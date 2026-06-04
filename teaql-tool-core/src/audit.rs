use std::collections::HashMap;

/// Identifies a specific tool module in the TeaQL ecosystem.
/// Used as the key for per-module audit configuration.
/// Application layer can only reference these predefined modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Module {
    // IO modules (default: Full audit)
    Http,
    File,
    Cmd,
    Email,
    Kv,

    // Security modules (default: Summary audit)
    Crypto,
    Jwt,

    // Computation modules (default: Silent)
    Time,
    Id,
    Text,
    Decimal,
    Money,
    Json,
    Regex,
    Codec,
    List,
    Map,
    Diff,
    Url,
    Validate,
    Color,
    Unit,
    DateRange,
    Desensitize,
    Filter,
    Tree,
    System,
}

/// Controls how much detail is captured for a module's operations.
/// Application layer picks from this closed set — no custom implementations allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuditLevel {
    /// No output at all. Maximum performance.
    Silent,

    /// Operation type + elapsed time only.
    /// Example: `[HTTP GET] 127ms OK`
    Summary,

    /// Operation type + comment + target + elapsed + trace ID.
    /// Example: `[HTTP GET] Intent: [Sync tasks] URL: https://... 127ms OK Trace: abc-123`
    Full,

    /// Full detail + captured request/response payload (truncated).
    /// For debugging and compliance-critical environments.
    FullWithPayload {
        max_bytes: usize,
    },
}

/// Per-module audit configuration.
/// Application layer can only select from predefined presets or
/// set individual module levels. No raw logging API is exposed.
#[derive(Debug, Clone)]
pub struct AuditConfig {
    levels: HashMap<Module, AuditLevel>,
    default_io_level: AuditLevel,
    default_compute_level: AuditLevel,
}

impl AuditConfig {
    /// Create a new AuditConfig with explicit default levels.
    pub fn new(io_level: AuditLevel, compute_level: AuditLevel) -> Self {
        Self {
            levels: HashMap::new(),
            default_io_level: io_level,
            default_compute_level: compute_level,
        }
    }

    // ─── Presets ───────────────────────────────────────────────

    /// Everything silent. Use in CI/test environments or when
    /// you want zero audit overhead.
    pub fn silent_all() -> Self {
        Self {
            levels: HashMap::new(),
            default_io_level: AuditLevel::Silent,
            default_compute_level: AuditLevel::Silent,
        }
    }

    /// Production defaults: IO = Full, Security = Summary, Compute = Silent.
    pub fn production() -> Self {
        let mut cfg = Self {
            levels: HashMap::new(),
            default_io_level: AuditLevel::Full,
            default_compute_level: AuditLevel::Silent,
        };
        cfg.levels.insert(Module::Crypto, AuditLevel::Summary);
        cfg.levels.insert(Module::Jwt, AuditLevel::Summary);
        cfg
    }

    /// Focus on a single module with Full audit, silence everything else.
    /// Ideal for development: only see what you care about.
    pub fn focus_on(module: Module) -> Self {
        let mut cfg = Self::silent_all();
        cfg.levels.insert(module, AuditLevel::Full);
        cfg
    }

    /// Focus on multiple modules, silence everything else.
    pub fn focus_on_many(modules: &[Module]) -> Self {
        let mut cfg = Self::silent_all();
        for m in modules {
            cfg.levels.insert(*m, AuditLevel::Full);
        }
        cfg
    }

    /// All IO modules at Full, all compute modules Silent.
    pub fn io_only() -> Self {
        Self {
            levels: HashMap::new(),
            default_io_level: AuditLevel::Full,
            default_compute_level: AuditLevel::Silent,
        }
    }

    /// Everything at Full. Use for deep debugging sessions.
    pub fn verbose_all() -> Self {
        Self {
            levels: HashMap::new(),
            default_io_level: AuditLevel::Full,
            default_compute_level: AuditLevel::Full,
        }
    }

    // ─── Per-module overrides ──────────────────────────────────

    /// Set a specific module's audit level. Returns self for chaining.
    pub fn enable(mut self, module: Module, level: AuditLevel) -> Self {
        self.levels.insert(module, level);
        self
    }

    /// Silence a specific module. Returns self for chaining.
    pub fn silence(mut self, module: Module) -> Self {
        self.levels.insert(module, AuditLevel::Silent);
        self
    }

    // ─── Runtime query (used by framework infra layer only) ───

    /// Returns the effective audit level for a given module.
    /// Framework infrastructure calls this internally to decide
    /// whether to log. Application code never calls this.
    pub fn level_for(&self, module: Module) -> AuditLevel {
        if let Some(&level) = self.levels.get(&module) {
            return level;
        }
        if Self::is_io_module(module) {
            self.default_io_level
        } else {
            self.default_compute_level
        }
    }

    /// Returns true if the given module should produce any output.
    pub fn is_active(&self, module: Module) -> bool {
        self.level_for(module) != AuditLevel::Silent
    }

    fn is_io_module(module: Module) -> bool {
        matches!(
            module,
            Module::Http
                | Module::File
                | Module::Cmd
                | Module::Email
                | Module::Kv
                | Module::Crypto
                | Module::Jwt
        )
    }
}

impl Default for AuditConfig {
    /// Sensible defaults: IO at Full, Compute at Silent.
    fn default() -> Self {
        Self::production()
    }
}
