use std::fmt::Write;
use std::time::Duration;

use crate::audit::{AuditLevel, Module};

/// A key-value pair for module-specific audit details.
pub struct AuditDetail<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> AuditDetail<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

/// Formats structured audit log entries based on the configured audit level.
/// This is the single source of truth for all audit output formats.
/// Application layer code never calls this directly — only the framework
/// infrastructure layer uses it inside `IntoFuture` and `MustComment` internals.
pub struct AuditFormatter;

impl AuditFormatter {
    /// Format an audit log entry. Returns `None` if the level is `Silent`.
    pub fn format(
        module: Module,
        level: AuditLevel,
        timestamp: &str,
        trace_id: &str,
        user: &str,
        operation: &str,
        comment: &str,
        elapsed: Duration,
        status: &str,
        details: &[AuditDetail<'_>],
    ) -> Option<String> {
        match level {
            AuditLevel::Silent => None,

            AuditLevel::Summary => {
                Some(format!(
                    "{} | {} | {} | {} | {} | {}ms | {}",
                    timestamp,
                    trace_id,
                    user,
                    Self::module_tag(module),
                    operation,
                    elapsed.as_millis(),
                    status,
                ))
            }

            AuditLevel::Full => {
                let mut out = format!(
                    "{} | {} | {} | {} | {} | {}ms | {}",
                    timestamp,
                    trace_id,
                    user,
                    Self::module_tag(module),
                    operation,
                    elapsed.as_millis(),
                    status,
                );
                let _ = write!(out, "\n  Intent: {}", comment);
                for d in details {
                    let _ = write!(out, "\n  {}: {}", d.key, d.value);
                }
                Some(out)
            }

            AuditLevel::FullWithPayload { max_bytes } => {
                let mut out = format!(
                    "{} | {} | {} | {} | {} | {}ms | {}",
                    timestamp,
                    trace_id,
                    user,
                    Self::module_tag(module),
                    operation,
                    elapsed.as_millis(),
                    status,
                );
                let _ = write!(out, "\n  Intent: {}", comment);
                for d in details {
                    let value = if d.value.len() > max_bytes {
                        format!("{}... (truncated to {}B)", &d.value[..max_bytes], max_bytes)
                    } else {
                        d.value.to_string()
                    };
                    let _ = write!(out, "\n  {}: {}", d.key, value);
                }
                Some(out)
            }
        }
    }

    /// Returns the uppercase tag for a module, used in log output.
    pub fn module_tag(module: Module) -> &'static str {
        match module {
            Module::Http => "HTTP",
            Module::File => "FILE",
            Module::Cmd => "CMD",
            Module::Email => "EMAIL",
            Module::Kv => "KV",
            Module::Crypto => "CRYPTO",
            Module::Jwt => "JWT",
            Module::Time => "TIME",
            Module::Id => "ID",
            Module::Text => "TEXT",
            Module::Decimal => "DECIMAL",
            Module::Money => "MONEY",
            Module::Json => "JSON",
            Module::Regex => "REGEX",
            Module::Codec => "CODEC",
            Module::List => "LIST",
            Module::Map => "MAP",
            Module::Diff => "DIFF",
            Module::Url => "URL",
            Module::Validate => "VALIDATE",
            Module::Color => "COLOR",
            Module::Unit => "UNIT",
            Module::DateRange => "DATERANGE",
            Module::Desensitize => "DESENSITIZE",
            Module::Filter => "FILTER",
            Module::Tree => "TREE",
            Module::System => "SYSTEM",
            _ => "UNKNOWN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TS: &str = "2026-06-04T09:15:00Z";
    const TRACE: &str = "a1b2c3d4";
    const USER: &str = "philip";

    fn print_module_card(
        module: Module,
        operation: &str,
        comment: &str,
        elapsed: Duration,
        status: &str,
        details: &[AuditDetail<'_>],
        payload_details: &[AuditDetail<'_>],
    ) {
        let tag = AuditFormatter::module_tag(module);
        println!("═══════════════════════════════════════════════════");
        println!("Module: {} | {:?}", tag, module);
        println!("═══════════════════════════════════════════════════");

        // Silent
        let result = AuditFormatter::format(
            module, AuditLevel::Silent, TS, TRACE, USER,
            operation, comment, elapsed, status, details,
        );
        println!("\n[Silent]\n(no output)");
        assert!(result.is_none());

        // Summary
        let result = AuditFormatter::format(
            module, AuditLevel::Summary, TS, TRACE, USER,
            operation, comment, elapsed, status, details,
        ).unwrap();
        println!("\n[Summary]\n{}", result);
        assert!(!result.contains("Intent:"));

        // Full
        let result = AuditFormatter::format(
            module, AuditLevel::Full, TS, TRACE, USER,
            operation, comment, elapsed, status, details,
        ).unwrap();
        println!("\n[Full]\n{}", result);
        assert!(result.contains("Intent:"));

        // FullWithPayload
        let result = AuditFormatter::format(
            module, AuditLevel::FullWithPayload { max_bytes: 128 }, TS, TRACE, USER,
            operation, comment, elapsed, status, payload_details,
        ).unwrap();
        println!("\n[FullWithPayload]\n{}", result);
        assert!(result.contains("Intent:"));

        println!();
    }

    #[test]
    fn audit_format_http() {
        print_module_card(
            Module::Http, "GET", "Fetch exchange rates for settlement",
            Duration::from_millis(127), "200 OK",
            &[AuditDetail::new("URL", "https://api.partner.com/v2/rates?base=USD")],
            &[
                AuditDetail::new("URL", "https://api.partner.com/v2/rates?base=USD"),
                AuditDetail::new("Response", r#"{"USD":1.0,"EUR":0.92,"CNY":7.24}"#),
            ],
        );
    }

    #[test]
    fn audit_format_file() {
        print_module_card(
            Module::File, "READ", "Load tenant-specific encryption key",
            Duration::from_millis(3), "OK 2048B",
            &[AuditDetail::new("Path", "/etc/teaql/tenants/1024/secret.pem")],
            &[
                AuditDetail::new("Path", "/etc/teaql/tenants/1024/secret.pem"),
                AuditDetail::new("Content", "[binary 2048 bytes]"),
            ],
        );
    }

    #[test]
    fn audit_format_cmd() {
        print_module_card(
            Module::Cmd, "exec", "Generate PDF invoice via wkhtmltopdf",
            Duration::from_millis(1200), "Exit(0)",
            &[AuditDetail::new("Command", "wkhtmltopdf /tmp/invoice.html /tmp/invoice.pdf")],
            &[
                AuditDetail::new("Command", "wkhtmltopdf /tmp/invoice.html /tmp/invoice.pdf"),
                AuditDetail::new("Stdout", "Loading page (1/2)\nPrinting pages (2/2)\nDone"),
            ],
        );
    }

    #[test]
    fn audit_format_time() {
        print_module_card(
            Module::Time, "today.add_days(7)", "Calculate 7-day grace period for overdue payment",
            Duration::from_nanos(800), "OK",
            &[
                AuditDetail::new("Timezone", "Asia/Shanghai"),
                AuditDetail::new("Result", "2026-06-11"),
            ],
            &[
                AuditDetail::new("Timezone", "Asia/Shanghai"),
                AuditDetail::new("Result", "2026-06-11"),
            ],
        );
    }

    #[test]
    fn audit_format_id() {
        print_module_card(
            Module::Id, "uuid_v7", "Generate idempotency key for payment callback",
            Duration::from_nanos(500), "OK",
            &[AuditDetail::new("Result", "019abc12-def3-7000-8000-000000000001")],
            &[AuditDetail::new("Result", "019abc12-def3-7000-8000-000000000001")],
        );
    }

    #[test]
    fn audit_format_money() {
        print_module_card(
            Module::Money, "add", "Calculate total including 6% VAT",
            Duration::from_nanos(200), "OK",
            &[
                AuditDetail::new("Input", "1000.00 USD + 60.00 USD"),
                AuditDetail::new("Result", "1060.00 USD"),
            ],
            &[
                AuditDetail::new("Input", "1000.00 USD + 60.00 USD"),
                AuditDetail::new("Result", "1060.00 USD"),
            ],
        );
    }

    #[test]
    fn audit_format_crypto() {
        print_module_card(
            Module::Crypto, "aes_gcm_encrypt", "Encrypt PII before persisting to database",
            Duration::from_millis(2), "OK",
            &[
                AuditDetail::new("Algorithm", "AES-256-GCM"),
                AuditDetail::new("Size", "256B -> 272B"),
            ],
            &[
                AuditDetail::new("Algorithm", "AES-256-GCM"),
                AuditDetail::new("Size", "256B -> 272B"),
                AuditDetail::new("Input", "[REDACTED]"),
            ],
        );
    }

    #[test]
    fn audit_format_json() {
        print_module_card(
            Module::Json, "parse", "Parse incoming webhook payload from Stripe",
            Duration::from_nanos(1500), "OK",
            &[AuditDetail::new("Size", "4096B")],
            &[
                AuditDetail::new("Size", "4096B"),
                AuditDetail::new("Content", r#"{"id":"evt_1234","type":"payment_intent.succeeded","data":{"object":{"id":"pi_abc","amount":10000,"currency":"usd","status":"succeeded"}}}"#),
            ],
        );
    }
}
