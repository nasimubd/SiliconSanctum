//! Zero-swap memory arbitration.
#![allow(clippy::missing_errors_doc)]

pub const BYTES_PER_GIB: u64 = 1_073_741_824;
pub const WIRED_LIMIT_BYTES: u64 = 10_400 * 1_048_576;
pub const DEFAULT_HEADROOM_BYTES: u64 = 512 * 1_048_576;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub wired_bytes: u64,
    pub available_bytes: u64,
    pub swap_used_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureLevel {
    Normal,
    Warning,
    Critical,
}

impl PressureLevel {
    #[must_use]
    pub const fn requires_eviction(self) -> bool {
        matches!(self, Self::Warning | Self::Critical)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArbiterError {
    EmptyModelId,
    ZeroValue { field: &'static str },
}

impl ModelId {
    pub fn new(value: impl Into<String>) -> Result<Self, ArbiterError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ArbiterError::EmptyModelId);
        }
        Ok(Self(value))
    }
}

impl ModelId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelResidency {
    Resident,
    Evicting,
    Evicted,
    Reloading,
}

impl ModelResidency {
    #[must_use]
    pub const fn occupies_memory(self) -> bool {
        matches!(self, Self::Resident | Self::Evicting)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFootprint {
    pub id: ModelId,
    pub weights_bytes: u64,
    pub kv_bytes: u64,
    pub priority: u8,
    pub residency: ModelResidency,
}

impl ModelFootprint {
    pub fn new(
        id: ModelId,
        weights_bytes: u64,
        kv_bytes: u64,
        priority: u8,
    ) -> Result<Self, ArbiterError> {
        if weights_bytes == 0 {
            return Err(ArbiterError::ZeroValue {
                field: "weights_bytes",
            });
        }
        Ok(Self {
            id,
            weights_bytes,
            kv_bytes,
            priority,
            residency: ModelResidency::Resident,
        })
    }
}

impl ModelFootprint {
    #[must_use]
    pub const fn total_bytes(&self) -> u64 {
        self.weights_bytes.saturating_add(self.kv_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArbiterPolicy {
    pub wired_limit_bytes: u64,
    pub recovery_bytes: u64,
    pub pressure_reserve_bytes: u64,
}

impl Default for ArbiterPolicy {
    fn default() -> Self {
        Self {
            wired_limit_bytes: WIRED_LIMIT_BYTES,
            recovery_bytes: DEFAULT_HEADROOM_BYTES,
            pressure_reserve_bytes: DEFAULT_HEADROOM_BYTES,
        }
    }
}

impl ArbiterPolicy {
    pub fn new(
        wired_limit_bytes: u64,
        recovery_bytes: u64,
        pressure_reserve_bytes: u64,
    ) -> Result<Self, ArbiterError> {
        if wired_limit_bytes == 0 {
            return Err(ArbiterError::ZeroValue {
                field: "wired_limit_bytes",
            });
        }
        Ok(Self {
            wired_limit_bytes,
            recovery_bytes,
            pressure_reserve_bytes,
        })
    }
}

impl ArbiterPolicy {
    #[must_use]
    pub const fn eviction_threshold(self) -> u64 {
        self.wired_limit_bytes
            .saturating_sub(self.pressure_reserve_bytes)
    }
}

impl ArbiterPolicy {
    #[must_use]
    pub const fn reload_threshold(self) -> u64 {
        self.eviction_threshold()
            .saturating_sub(self.recovery_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArbiterAction {
    Hold,
    Evict,
    Reload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionReason {
    WithinBudget,
    WiredCeiling,
    PressureWarning,
    PressureCritical,
    RecoveryHeadroom,
    SwapDetected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArbiterDecision {
    pub action: ArbiterAction,
    pub reason: DecisionReason,
}

impl ArbiterDecision {
    #[must_use]
    pub const fn hold() -> Self {
        Self {
            action: ArbiterAction::Hold,
            reason: DecisionReason::WithinBudget,
        }
    }
}

impl ArbiterDecision {
    #[must_use]
    pub const fn evict(reason: DecisionReason) -> Self {
        Self {
            action: ArbiterAction::Evict,
            reason,
        }
    }
}

impl ArbiterDecision {
    #[must_use]
    pub const fn reload() -> Self {
        Self {
            action: ArbiterAction::Reload,
            reason: DecisionReason::RecoveryHeadroom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryBudget {
    pub usable_bytes: u64,
    pub model_bytes: u64,
    pub kv_cache_bytes: u64,
    pub reserve_bytes: u64,
}

impl MemoryBudget {
    #[must_use]
    pub const fn total_allocated(self) -> u64 {
        self.model_bytes.saturating_add(self.kv_cache_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetRequest {
    pub model_bytes: u64,
    pub requested_kv_bytes: u64,
}

impl BudgetRequest {
    #[must_use]
    pub const fn new(model_bytes: u64, requested_kv_bytes: u64) -> Self {
        Self {
            model_bytes,
            requested_kv_bytes,
        }
    }
}

#[must_use]
pub fn calculate_budget(
    snapshot: MemorySnapshot,
    policy: ArbiterPolicy,
    request: BudgetRequest,
) -> MemoryBudget {
    let usable = policy
        .wired_limit_bytes
        .saturating_sub(snapshot.wired_bytes)
        .saturating_sub(policy.pressure_reserve_bytes);
    let kv = usable
        .saturating_sub(request.model_bytes)
        .min(request.requested_kv_bytes);
    MemoryBudget {
        usable_bytes: usable,
        model_bytes: request.model_bytes.min(usable),
        kv_cache_bytes: kv,
        reserve_bytes: policy.pressure_reserve_bytes,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DecisionEngine {
    policy: ArbiterPolicy,
}

impl DecisionEngine {
    #[must_use]
    pub const fn new(policy: ArbiterPolicy) -> Self {
        Self { policy }
    }
}

impl DecisionEngine {
    #[must_use]
    pub const fn policy(self) -> ArbiterPolicy {
        self.policy
    }
}

impl DecisionEngine {
    #[must_use]
    pub const fn evaluate(
        self,
        snapshot: MemorySnapshot,
        pressure: PressureLevel,
        has_evicted: bool,
    ) -> ArbiterDecision {
        if snapshot.swap_used_bytes > 0 {
            return ArbiterDecision::evict(DecisionReason::SwapDetected);
        }
        if matches!(pressure, PressureLevel::Critical) {
            return ArbiterDecision::evict(DecisionReason::PressureCritical);
        }
        if snapshot.wired_bytes >= self.policy.eviction_threshold() {
            return ArbiterDecision::evict(DecisionReason::WiredCeiling);
        }
        if matches!(pressure, PressureLevel::Warning) {
            return ArbiterDecision::evict(DecisionReason::PressureWarning);
        }
        if has_evicted && snapshot.wired_bytes <= self.policy.reload_threshold() {
            return ArbiterDecision::reload();
        }
        ArbiterDecision::hold()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvictionCandidate {
    pub model: ModelFootprint,
    pub last_used_epoch: u64,
}

impl EvictionCandidate {
    #[must_use]
    pub const fn reclaimable_bytes(&self) -> u64 {
        self.model.total_bytes()
    }
}

#[must_use]
pub fn select_eviction_candidate(models: &[EvictionCandidate]) -> Option<&EvictionCandidate> {
    models
        .iter()
        .filter(|item| item.model.residency == ModelResidency::Resident)
        .min_by_key(|item| (item.model.priority, item.last_used_epoch))
}

#[must_use]
pub fn select_reload_candidate(models: &[EvictionCandidate]) -> Option<&EvictionCandidate> {
    models
        .iter()
        .filter(|item| item.model.residency == ModelResidency::Evicted)
        .max_by_key(|item| (item.model.priority, std::cmp::Reverse(item.last_used_epoch)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArbiterEvent {
    Sampled(MemorySnapshot),
    Decision(ArbiterDecision),
    EvictionRequested(ModelId),
    ReloadRequested(ModelId),
}

impl MemorySnapshot {
    #[must_use]
    pub const fn new(wired_bytes: u64, available_bytes: u64, swap_used_bytes: u64) -> Self {
        Self {
            wired_bytes,
            available_bytes,
            swap_used_bytes,
        }
    }

    #[must_use]
    pub const fn wired_bytes(self) -> u64 {
        self.wired_bytes
    }
    #[must_use]
    pub const fn available_bytes(self) -> u64 {
        self.available_bytes
    }
    #[must_use]
    pub const fn swap_used_bytes(self) -> u64 {
        self.swap_used_bytes
    }
}
