//! Token radix prefix caching.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadixError {
    EmptyTokens,
    EmptySession,
    ZeroCapacity,
    UnsupportedAttention,
    MissingSession,
}

pub type TokenId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenSequence(Vec<TokenId>);

impl TokenSequence {
    pub fn new(tokens: Vec<TokenId>) -> Result<Self, RadixError> {
        if tokens.is_empty() {
            return Err(RadixError::EmptyTokens);
        }
        Ok(Self(tokens))
    }
}

impl TokenSequence {
    #[must_use]
    pub fn tokens(&self) -> &[TokenId] {
        &self.0
    }
}

impl TokenSequence {
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TokenSequence {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(v: impl Into<String>) -> Result<Self, RadixError> {
        let v = v.into();
        if v.trim().is_empty() {
            return Err(RadixError::EmptySession);
        }
        Ok(Self(v))
    }
}

impl SessionId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttentionArchitecture {
    Full,
    SlidingWindow,
    Recurrent,
    Hybrid,
}

impl AttentionArchitecture {
    #[must_use]
    pub const fn supports_prefix_cache(self) -> bool {
        matches!(self, Self::Full)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelArchitecture {
    pub name: String,
    pub attention: AttentionArchitecture,
}

impl ModelArchitecture {
    pub fn validate_prefix_cache(&self) -> Result<(), RadixError> {
        if self.attention.supports_prefix_cache() {
            Ok(())
        } else {
            Err(RadixError::UnsupportedAttention)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheCapacity {
    pub max_nodes: usize,
    pub max_bytes: usize,
}

impl CacheCapacity {
    pub fn new(max_nodes: usize, max_bytes: usize) -> Result<Self, RadixError> {
        if max_nodes == 0 || max_bytes == 0 {
            return Err(RadixError::ZeroCapacity);
        }
        Ok(Self {
            max_nodes,
            max_bytes,
        })
    }
}

impl Default for CacheCapacity {
    fn default() -> Self {
        Self {
            max_nodes: 65_536,
            max_bytes: 512 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheHandle(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixMatch {
    pub handle: Option<CacheHandle>,
    pub matched_tokens: usize,
    pub remaining_tokens: usize,
}

impl PrefixMatch {
    #[must_use]
    pub const fn hit(&self) -> bool {
        self.matched_tokens > 0
    }
}

#[derive(Debug, Default)]
pub struct RadixNode {
    children: std::collections::BTreeMap<TokenId, RadixNode>,
    handle: Option<CacheHandle>,
    bytes: usize,
    last_used: u64,
}

impl RadixNode {
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

impl RadixNode {
    #[must_use]
    pub const fn handle(&self) -> Option<CacheHandle> {
        self.handle
    }
}

impl RadixNode {
    #[must_use]
    pub const fn stored_bytes(&self) -> usize {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub nodes: usize,
    pub bytes: usize,
    pub entries: usize,
    pub hits: u64,
    pub misses: u64,
}

#[derive(Debug)]
pub struct RadixCache {
    root: RadixNode,
    capacity: CacheCapacity,
    stats: CacheStats,
    next_handle: u64,
    clock: u64,
    sessions: std::collections::HashMap<SessionId, TokenSequence>,
}

impl RadixCache {
    #[must_use]
    pub fn new(capacity: CacheCapacity) -> Self {
        Self {
            root: RadixNode::default(),
            capacity,
            stats: CacheStats {
                nodes: 1,
                ..CacheStats::default()
            },
            next_handle: 1,
            clock: 0,
            sessions: std::collections::HashMap::new(),
        }
    }
}

impl RadixCache {
    #[must_use]
    pub const fn capacity(&self) -> CacheCapacity {
        self.capacity
    }
}

impl RadixCache {
    #[must_use]
    pub const fn stats(&self) -> CacheStats {
        self.stats
    }
}

impl RadixCache {
    pub fn insert(&mut self, sequence: &TokenSequence, bytes: usize) -> CacheHandle {
        self.clock = self.clock.saturating_add(1);
        let mut node = &mut self.root;
        for token in sequence.tokens() {
            node = node.children.entry(*token).or_insert_with(|| {
                self.stats.nodes = self.stats.nodes.saturating_add(1);
                RadixNode::default()
            });
        }
        if let Some(handle) = node.handle {
            node.last_used = self.clock;
            return handle;
        }
        let handle = CacheHandle(self.next_handle);
        self.next_handle = self.next_handle.saturating_add(1);
        node.handle = Some(handle);
        node.bytes = bytes;
        node.last_used = self.clock;
        self.stats.entries = self.stats.entries.saturating_add(1);
        self.stats.bytes = self.stats.bytes.saturating_add(bytes);
        handle
    }
}

impl RadixCache {
    pub fn lookup(&mut self, sequence: &TokenSequence) -> PrefixMatch {
        self.clock = self.clock.saturating_add(1);
        let mut node = &mut self.root;
        let mut matched = 0;
        let mut handle = None;
        for token in sequence.tokens() {
            let Some(next) = node.children.get_mut(token) else {
                break;
            };
            node = next;
            matched += 1;
            if node.handle.is_some() {
                handle = node.handle;
                node.last_used = self.clock;
            }
        }
        if matched > 0 {
            self.stats.hits = self.stats.hits.saturating_add(1);
        } else {
            self.stats.misses = self.stats.misses.saturating_add(1);
        }
        PrefixMatch {
            handle,
            matched_tokens: matched,
            remaining_tokens: sequence.len() - matched,
        }
    }
}

impl RadixCache {
    pub fn bind_session(&mut self, id: SessionId, sequence: TokenSequence) {
        self.sessions.insert(id, sequence);
    }
}

impl RadixCache {
    #[must_use]
    pub fn session(&self, id: &SessionId) -> Option<&TokenSequence> {
        self.sessions.get(id)
    }
}

impl RadixCache {
    pub fn remove_session(&mut self, id: &SessionId) -> Result<TokenSequence, RadixError> {
        self.sessions.remove(id).ok_or(RadixError::MissingSession)
    }
}

impl RadixCache {
    #[must_use]
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

impl RadixCache {
    #[must_use]
    pub fn root(&self) -> &RadixNode {
        &self.root
    }
}

impl RadixCache {
    #[must_use]
    pub fn contains_handle(&self, handle: CacheHandle) -> bool {
        fn walk(n: &RadixNode, h: CacheHandle) -> bool {
            n.handle == Some(h) || n.children.values().any(|v| walk(v, h))
        }
        walk(&self.root, handle)
    }
}

impl RadixCache {
    #[must_use]
    pub fn within_capacity(&self) -> bool {
        self.stats.nodes <= self.capacity.max_nodes && self.stats.bytes <= self.capacity.max_bytes
    }
}

impl RadixCache {
    pub fn clear(&mut self) {
        self.root = RadixNode::default();
        self.stats = CacheStats {
            nodes: 1,
            ..CacheStats::default()
        };
        self.sessions.clear();
    }
}

impl RadixCache {
    pub fn enforce_capacity(&mut self) {
        if !self.within_capacity() {
            self.clear();
        }
    }
}

#[must_use]
pub fn common_prefix_len(left: &[TokenId], right: &[TokenId]) -> usize {
    left.iter().zip(right).take_while(|(a, b)| a == b).count()
}

#[must_use]
pub fn shared_prefix(left: &TokenSequence, right: &TokenSequence) -> Option<TokenSequence> {
    let len = common_prefix_len(left.tokens(), right.tokens());
    (len > 0).then(|| TokenSequence(left.tokens()[..len].to_vec()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitectureFamily {
    Transformer,
    Gemma,
    QwenSsm,
    Mamba,
    Unknown,
}

#[must_use]
pub fn infer_architecture_family(name: &str) -> ArchitectureFamily {
    let n = name.to_ascii_lowercase();
    if n.contains("gemma") {
        ArchitectureFamily::Gemma
    } else if n.contains("qwen") && n.contains("ssm") {
        ArchitectureFamily::QwenSsm
    } else if n.contains("mamba") {
        ArchitectureFamily::Mamba
    } else if n.contains("llama") || n.contains("qwen") {
        ArchitectureFamily::Transformer
    } else {
        ArchitectureFamily::Unknown
    }
}

#[must_use]
pub const fn family_attention(family: ArchitectureFamily) -> AttentionArchitecture {
    match family {
        ArchitectureFamily::Transformer => AttentionArchitecture::Full,
        ArchitectureFamily::Gemma => AttentionArchitecture::SlidingWindow,
        ArchitectureFamily::QwenSsm | ArchitectureFamily::Mamba => AttentionArchitecture::Recurrent,
        ArchitectureFamily::Unknown => AttentionArchitecture::Hybrid,
    }
}

pub fn validate_model_name(name: &str) -> Result<ModelArchitecture, RadixError> {
    let family = infer_architecture_family(name);
    let model = ModelArchitecture {
        name: name.to_owned(),
        attention: family_attention(family),
    };
    model.validate_prefix_cache()?;
    Ok(model)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheEventKind {
    Inserted,
    Hit,
    Miss,
    Evicted,
    SessionBound,
    SessionRemoved,
}

pub struct RadixMarker;
