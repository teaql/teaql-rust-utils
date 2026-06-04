use std::collections::{HashMap, HashSet};

use crate::audit::{AuditConfig, AuditLevel, Module};

// ─── Whitelist ────────────────────────────────────────────────

/// The complete, fixed set of allowed TEAQL_ environment variable names.
/// Any TEAQL_ prefixed env var not in this list causes immediate process exit.
const ALLOWED_ENV_VARS: &[&str] = &[
    "TEAQL_AUDIT",
    "TEAQL_SQL",
    "TEAQL_SQL_TABLES",
    "TEAQL_TOOL",
    "TEAQL_TOOL_FOCUS",
    "TEAQL_SINK",
];

/// Allowed values for level-type env vars. Prefixed with underscore
/// to avoid collision with user-defined entity/table names.
const ALLOWED_LEVELS: &[&str] = &["_silent", "_summary", "_full"];

/// Allowed values for TEAQL_SINK.
const ALLOWED_SINKS: &[&str] = &["_stdout", "_file", "_both"];

/// All valid module names for TEAQL_TOOL_FOCUS.
const ALLOWED_MODULES: &[(&str, Module)] = &[
    ("http", Module::Http),
    ("file", Module::File),
    ("cmd", Module::Cmd),
    ("email", Module::Email),
    ("kv", Module::Kv),
    ("crypto", Module::Crypto),
    ("jwt", Module::Jwt),
    ("time", Module::Time),
    ("id", Module::Id),
    ("text", Module::Text),
    ("decimal", Module::Decimal),
    ("money", Module::Money),
    ("json", Module::Json),
    ("regex", Module::Regex),
    ("codec", Module::Codec),
    ("list", Module::List),
    ("map", Module::Map),
    ("diff", Module::Diff),
    ("url", Module::Url),
    ("validate", Module::Validate),
    ("color", Module::Color),
    ("unit", Module::Unit),
    ("daterange", Module::DateRange),
    ("desensitize", Module::Desensitize),
    ("filter", Module::Filter),
    ("tree", Module::Tree),
    ("system", Module::System),
];

// ─── Output sink ──────────────────────────────────────────────

/// Where audit output is written. Controlled by TEAQL_SINK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditSink {
    Stdout,
    File,
    Both,
}

impl Default for AuditSink {
    fn default() -> Self {
        AuditSink::Both
    }
}

// ─── Env config result ────────────────────────────────────────

/// The fully resolved configuration parsed from environment variables.
/// Constructed by `AuditConfig::from_env()` or `AuditConfig::from_env_with_tables()`.
#[derive(Debug, Clone)]
pub struct EnvAuditConfig {
    /// Core audit config with module levels.
    pub config: AuditConfig,
    /// Global entity audit level (from TEAQL_AUDIT).
    pub entity_level: AuditLevel,
    /// Global SQL log level (from TEAQL_SQL).
    pub sql_level: AuditLevel,
    /// SQL table include filter (from TEAQL_SQL_TABLES). None = all tables.
    pub sql_tables: Option<HashSet<String>>,
    /// Output sink (from TEAQL_SINK).
    pub sink: AuditSink,
}

impl EnvAuditConfig {
    /// Check if SQL logging is active for a given table name.
    pub fn sql_active_for(&self, table: &str) -> bool {
        if self.sql_level == AuditLevel::Silent {
            return false;
        }
        match &self.sql_tables {
            Some(tables) => tables.contains(table),
            None => true, // No filter = all tables
        }
    }
}

// ─── Parsing ──────────────────────────────────────────────────

fn parse_level(value: &str, var_name: &str) -> AuditLevel {
    match value {
        "_silent" => AuditLevel::Silent,
        "_summary" => AuditLevel::Summary,
        "_full" => AuditLevel::Full,
        other => {
            eprintln!(
                "\nFATAL: Invalid value \"{}\" for environment variable \"{}\"\n\
                 Allowed values: {}\n\n\
                 Application refused to start.\n",
                other,
                var_name,
                ALLOWED_LEVELS.join(", "),
            );
            std::process::exit(1);
        }
    }
}

fn parse_sink(value: &str) -> AuditSink {
    match value {
        "_stdout" => AuditSink::Stdout,
        "_file" => AuditSink::File,
        "_both" => AuditSink::Both,
        other => {
            eprintln!(
                "\nFATAL: Invalid value \"{}\" for environment variable \"TEAQL_SINK\"\n\
                 Allowed values: {}\n\n\
                 Application refused to start.\n",
                other,
                ALLOWED_SINKS.join(", "),
            );
            std::process::exit(1);
        }
    }
}

fn parse_module_list(value: &str) -> Vec<Module> {
    let module_map: HashMap<&str, Module> = ALLOWED_MODULES.iter().copied().collect();
    let available: Vec<&str> = ALLOWED_MODULES.iter().map(|(name, _)| *name).collect();

    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|name| {
            *module_map.get(name).unwrap_or_else(|| {
                eprintln!(
                    "\nFATAL: Unknown module \"{}\" in TEAQL_TOOL_FOCUS\n\
                     Available modules: {}\n\n\
                     Application refused to start.\n",
                    name,
                    available.join(", "),
                );
                std::process::exit(1);
            })
        })
        .collect()
}

fn parse_table_list(value: &str, known_tables: &[&str]) -> HashSet<String> {
    let known_set: HashSet<&str> = known_tables.iter().copied().collect();
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|name| {
            if !known_set.contains(name) {
                eprintln!(
                    "\nFATAL: Unknown table \"{}\" in TEAQL_SQL_TABLES\n\
                     Available tables: {}\n\n\
                     Application refused to start.\n",
                    name,
                    known_tables.join(", "),
                );
                std::process::exit(1);
            }
            name.to_string()
        })
        .collect()
}

// ─── Whitelist enforcement ────────────────────────────────────

/// The set of TEAQL_ prefixes that belong to the audit config namespace.
/// Any env var starting with one of these prefixes must exactly match
/// an entry in ALLOWED_ENV_VARS, or the process exits.
const AUDIT_PREFIXES: &[&str] = &[
    "TEAQL_AUDIT",
    "TEAQL_SQL",
    "TEAQL_TOOL",
    "TEAQL_SINK",
    "TEAQL_SCHEMA",
];

/// Scan environment variables in the audit namespace.
/// If any matches an audit prefix but is not in the exact whitelist,
/// print an error and exit immediately.
/// Non-audit TEAQL_ vars (e.g. TEAQL_ENDPOINT_PREFIX) are ignored.
fn enforce_env_whitelist() {
    let allowed: HashSet<&str> = ALLOWED_ENV_VARS.iter().copied().collect();

    for (key, _) in std::env::vars() {
        let in_audit_namespace = AUDIT_PREFIXES.iter().any(|prefix| key.starts_with(prefix));
        if in_audit_namespace && !allowed.contains(key.as_str()) {
            // Try to suggest a close match
            let suggestion = ALLOWED_ENV_VARS
                .iter()
                .min_by_key(|v| levenshtein(&key, v))
                .unwrap();

            eprintln!(
                "\nFATAL: Unknown environment variable \"{}\"\n\
                 Did you mean \"{}\"?\n\n\
                 Allowed TEAQL audit variables:\n  {}\n\n\
                 Application refused to start.\n",
                key,
                suggestion,
                ALLOWED_ENV_VARS.join(", "),
            );
            std::process::exit(1);
        }
    }
}

/// Simple Levenshtein distance for typo suggestion.
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[m][n]
}

// ─── Public API ───────────────────────────────────────────────

/// Build an `EnvAuditConfig` from environment variables.
///
/// `known_tables` is the list of valid table names for this project,
/// typically provided by the generated code. Used to validate
/// TEAQL_SQL_TABLES values.
///
/// This function:
/// 1. Scans ALL env vars — panics if any unknown TEAQL_ var is found.
/// 2. Parses the 6 allowed env vars into a typed config.
/// 3. Validates all values against their respective whitelists.
///
/// If no TEAQL_ env vars are set, returns production defaults.
pub fn audit_config_from_env(known_tables: &[&str]) -> EnvAuditConfig {
    // Step 1: Reject unknown TEAQL_ env vars
    enforce_env_whitelist();

    // Step 2: Parse each env var (use defaults if not set)
    let entity_level = std::env::var("TEAQL_AUDIT")
        .map(|v| parse_level(&v, "TEAQL_AUDIT"))
        .unwrap_or(AuditLevel::Full);

    let sql_level = std::env::var("TEAQL_SQL")
        .map(|v| parse_level(&v, "TEAQL_SQL"))
        .unwrap_or(AuditLevel::Silent);

    let sql_tables: Option<HashSet<String>> = std::env::var("TEAQL_SQL_TABLES")
        .ok()
        .map(|v| parse_table_list(&v, known_tables));

    let tool_level = std::env::var("TEAQL_TOOL")
        .map(|v| parse_level(&v, "TEAQL_TOOL"))
        .unwrap_or(AuditLevel::Silent);

    let tool_focus: Option<Vec<Module>> = std::env::var("TEAQL_TOOL_FOCUS")
        .ok()
        .map(|v| parse_module_list(&v));

    let sink = std::env::var("TEAQL_SINK")
        .map(|v| parse_sink(&v))
        .unwrap_or(AuditSink::Both);

    // Step 3: Build the AuditConfig
    let config = match &tool_focus {
        Some(focused) => {
            // Focus mode: listed modules get _full, rest get tool_level
            let mut cfg = AuditConfig::new(tool_level, tool_level);
            for m in focused {
                cfg = cfg.enable(*m, AuditLevel::Full);
            }
            cfg
        }
        None => {
            // No focus: all modules use tool_level
            AuditConfig::new(tool_level, tool_level)
        }
    };

    EnvAuditConfig {
        config,
        entity_level,
        sql_level,
        sql_tables,
        sink,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("TEAQL_SQL", "TEAQL_SQL"), 0);
        assert_eq!(levenshtein("TEAQL_SQLL", "TEAQL_SQL"), 1);
        assert_eq!(levenshtein("TEAQL_AUDIT", "TEAQL_SQL"), 5);
    }

    #[test]
    fn test_parse_level() {
        assert_eq!(parse_level("_silent", "TEST"), AuditLevel::Silent);
        assert_eq!(parse_level("_summary", "TEST"), AuditLevel::Summary);
        assert_eq!(parse_level("_full", "TEST"), AuditLevel::Full);
    }

    #[test]
    fn test_parse_sink() {
        assert_eq!(parse_sink("_stdout"), AuditSink::Stdout);
        assert_eq!(parse_sink("_file"), AuditSink::File);
        assert_eq!(parse_sink("_both"), AuditSink::Both);
    }

    #[test]
    fn test_parse_module_list() {
        let modules = parse_module_list("http,money,crypto");
        assert_eq!(modules.len(), 3);
        assert_eq!(modules[0], Module::Http);
        assert_eq!(modules[1], Module::Money);
        assert_eq!(modules[2], Module::Crypto);
    }

    #[test]
    fn test_parse_table_list() {
        let tables = parse_table_list("task,task_status", &["task", "task_status", "task_execution_log"]);
        assert_eq!(tables.len(), 2);
        assert!(tables.contains("task"));
        assert!(tables.contains("task_status"));
    }

    #[test]
    fn test_sql_active_for() {
        let cfg = EnvAuditConfig {
            config: AuditConfig::production(),
            entity_level: AuditLevel::Full,
            sql_level: AuditLevel::Full,
            sql_tables: Some(["task".to_string()].into_iter().collect()),
            sink: AuditSink::Both,
        };
        assert!(cfg.sql_active_for("task"));
        assert!(!cfg.sql_active_for("task_status"));

        // No filter = all tables active
        let cfg_all = EnvAuditConfig {
            config: AuditConfig::production(),
            entity_level: AuditLevel::Full,
            sql_level: AuditLevel::Full,
            sql_tables: None,
            sink: AuditSink::Both,
        };
        assert!(cfg_all.sql_active_for("task"));
        assert!(cfg_all.sql_active_for("anything"));

        // Silent level = nothing active
        let cfg_silent = EnvAuditConfig {
            config: AuditConfig::production(),
            entity_level: AuditLevel::Full,
            sql_level: AuditLevel::Silent,
            sql_tables: None,
            sink: AuditSink::Both,
        };
        assert!(!cfg_silent.sql_active_for("task"));
    }
}
