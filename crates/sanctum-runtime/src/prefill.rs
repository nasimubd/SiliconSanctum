//! Chunked context prefill scheduling.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefillError {
    EmptyRequest,
    ZeroBudget,
    InvalidGeometry,
    InvalidThresholds,
    UnsafeScratch,
    InsufficientMemory,
    CriticalPressure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingDecision {
    Conservative,
    Throughput,
    Rejected,
}

impl SchedulingDecision {
    #[must_use]
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Conservative => "memory headroom requires 512-token chunks",
            Self::Throughput => "memory headroom permits 1024-token chunks",
            Self::Rejected => "prefill exceeds memory safety limits",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SchedulerMetrics {
    pub requests: u64,
    pub chunks: u64,
    pub rejected: u64,
    pub selected_512: u64,
    pub selected_1024: u64,
}

impl SchedulerMetrics {
    pub fn record_request(&mut self) {
        self.requests = self.requests.saturating_add(1);
    }

    pub fn record_chunks(&mut self, count: usize) {
        self.chunks = self.chunks.saturating_add(count as u64);
    }

    pub fn record_rejection(&mut self) {
        self.rejected = self.rejected.saturating_add(1);
    }

    pub fn record_step(&mut self, step: PrefillStep) {
        if step == PrefillStep::Tokens512 {
            self.selected_512 = self.selected_512.saturating_add(1);
        } else {
            self.selected_1024 = self.selected_1024.saturating_add(1);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefillEvent {
    Planned { chunks: usize, step: PrefillStep },
    Rejected(PrefillError),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct PrefillEventLog {
    capacity: usize,
    events: std::collections::VecDeque<PrefillEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCount(usize);

impl TokenCount {
    pub fn new(value: usize) -> Result<Self, PrefillError> {
        if value == 0 {
            return Err(PrefillError::EmptyRequest);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefillStep {
    Tokens512,
    Tokens1024,
}

impl PrefillStep {
    #[must_use]
    pub const fn tokens(self) -> usize {
        match self {
            Self::Tokens512 => 512,
            Self::Tokens1024 => 1024,
        }
    }

    #[must_use]
    pub const fn for_tokens(tokens: TokenCount) -> Self {
        if tokens.get() <= 512 {
            Self::Tokens512
        } else {
            Self::Tokens1024
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScratchBudget(u64);

impl ScratchBudget {
    pub const fn new(bytes: u64) -> Result<Self, PrefillError> {
        if bytes == 0 {
            return Err(PrefillError::ZeroBudget);
        }
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub total_bytes: u64,
    pub resident_bytes: u64,
    pub reserved_bytes: u64,
}

impl MemorySnapshot {
    #[must_use]
    pub const fn available_bytes(self) -> u64 {
        self.total_bytes
            .saturating_sub(self.resident_bytes)
            .saturating_sub(self.reserved_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KvQuantization {
    Bits4,
    Bits8,
    Bits16,
}

impl KvQuantization {
    #[must_use]
    pub const fn bits(self) -> u8 {
        match self {
            Self::Bits4 => 4,
            Self::Bits8 => 8,
            Self::Bits16 => 16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelGeometry {
    pub layers: u32,
    pub kv_heads: u32,
    pub head_dim: u32,
}

impl ModelGeometry {
    pub const fn new(layers: u32, kv_heads: u32, head_dim: u32) -> Result<Self, PrefillError> {
        if layers == 0 || kv_heads == 0 || head_dim == 0 {
            return Err(PrefillError::InvalidGeometry);
        }
        Ok(Self {
            layers,
            kv_heads,
            head_dim,
        })
    }

    #[must_use]
    pub const fn kv_bytes_per_token(self, quantization: KvQuantization) -> u64 {
        let elements = self.layers as u64 * self.kv_heads as u64 * self.head_dim as u64 * 2;
        elements
            .saturating_mul(quantization.bits() as u64)
            .div_ceil(8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TuningThresholds {
    pub constrained_bytes: u64,
    pub comfortable_bytes: u64,
}

impl TuningThresholds {
    pub const fn new(constrained_bytes: u64, comfortable_bytes: u64) -> Result<Self, PrefillError> {
        if constrained_bytes == 0 || constrained_bytes >= comfortable_bytes {
            return Err(PrefillError::InvalidThresholds);
        }
        Ok(Self {
            constrained_bytes,
            comfortable_bytes,
        })
    }
}

impl Default for TuningThresholds {
    fn default() -> Self {
        Self {
            constrained_bytes: 512 * 1024 * 1024,
            comfortable_bytes: 2 * 1024 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TuningPolicy {
    pub thresholds: TuningThresholds,
    pub scratch_budget: ScratchBudget,
}

impl TuningPolicy {
    pub const fn new(
        thresholds: TuningThresholds,
        scratch_budget: ScratchBudget,
    ) -> Result<Self, PrefillError> {
        if thresholds.constrained_bytes >= thresholds.comfortable_bytes {
            return Err(PrefillError::InvalidThresholds);
        }
        Ok(Self {
            thresholds,
            scratch_budget,
        })
    }
}

impl Default for TuningPolicy {
    fn default() -> Self {
        Self {
            thresholds: TuningThresholds::default(),
            scratch_budget: ScratchBudget(512 * 1024 * 1024),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefillRequest {
    pub tokens: TokenCount,
    pub geometry: ModelGeometry,
    pub quantization: KvQuantization,
}

impl PrefillRequest {
    pub const fn new(
        tokens: TokenCount,
        geometry: ModelGeometry,
        quantization: KvQuantization,
    ) -> Result<Self, PrefillError> {
        if tokens.get() == 0 {
            return Err(PrefillError::EmptyRequest);
        }
        Ok(Self {
            tokens,
            geometry,
            quantization,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefillChunk {
    pub offset: usize,
    pub end: usize,
}

impl PrefillChunk {
    #[must_use]
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.offset)
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefillPlan {
    pub step: PrefillStep,
    pub chunks: Vec<PrefillChunk>,
    pub scratch_bytes: u64,
    pub kv_bytes: u64,
}

impl PrefillPlan {
    #[must_use]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    #[must_use]
    pub fn planned_tokens(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.len()).sum()
    }

    #[must_use]
    pub const fn peak_scratch_bytes(&self) -> u64 {
        self.scratch_bytes
    }
}

#[derive(Debug, Clone)]
pub struct PrefillScheduler {
    policy: TuningPolicy,
}

impl PrefillScheduler {
    #[must_use]
    pub const fn new(policy: TuningPolicy) -> Self {
        Self { policy }
    }

    #[must_use]
    pub const fn constrained_step(&self) -> PrefillStep {
        PrefillStep::Tokens512
    }

    #[must_use]
    pub const fn comfortable_step(&self) -> PrefillStep {
        PrefillStep::Tokens1024
    }

    #[must_use]
    pub const fn select_step(
        &self,
        request: PrefillRequest,
        memory: MemorySnapshot,
    ) -> PrefillStep {
        if request.tokens.get() <= 512
            || memory.available_bytes() < self.policy.thresholds.comfortable_bytes
        {
            self.constrained_step()
        } else {
            self.comfortable_step()
        }
    }

    pub fn build_plan(
        &self,
        request: PrefillRequest,
        memory: MemorySnapshot,
    ) -> Result<PrefillPlan, PrefillError> {
        let step = self.select_step(request, memory);
        let scratch_bytes = estimate_scratch_bytes(step, request.geometry);
        if scratch_bytes > self.policy.scratch_budget.bytes() {
            return Err(PrefillError::UnsafeScratch);
        }
        let kv_bytes = request
            .geometry
            .kv_bytes_per_token(request.quantization)
            .saturating_mul(request.tokens.get() as u64);
        if kv_bytes.saturating_add(scratch_bytes) > memory.available_bytes() {
            return Err(PrefillError::InsufficientMemory);
        }
        Ok(PrefillPlan {
            step,
            chunks: partition_tokens(request.tokens, step),
            scratch_bytes,
            kv_bytes,
        })
    }
}

#[must_use]
pub fn partition_tokens(tokens: TokenCount, step: PrefillStep) -> Vec<PrefillChunk> {
    let mut chunks = Vec::new();
    let mut offset = 0;
    while offset < tokens.get() {
        let end = offset.saturating_add(step.tokens()).min(tokens.get());
        chunks.push(PrefillChunk { offset, end });
        offset = end;
    }
    chunks
}

#[must_use]
pub const fn estimate_scratch_bytes(step: PrefillStep, geometry: ModelGeometry) -> u64 {
    geometry
        .kv_bytes_per_token(KvQuantization::Bits16)
        .saturating_mul(step.tokens() as u64)
}
