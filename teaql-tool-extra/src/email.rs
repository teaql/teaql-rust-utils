use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use teaql_tool_core::{Result, TeaQLToolError};

#[derive(Debug, Clone)]
pub struct EmailTool;

impl EmailTool {
    pub fn new() -> Self { Self }

    pub fn send(&self, server: &str, user: &str, pass: &str, from: &str, to: &str, subject: &str, body: &str) -> Result<()> {
        let email = Message::builder()
            .from(from.parse().map_err(|_| TeaQLToolError::ParseError("Invalid from".into()))?)
            .to(to.parse().map_err(|_| TeaQLToolError::ParseError("Invalid to".into()))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(body))
            .map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;

        let creds = Credentials::new(user.to_owned(), pass.to_owned());

        let mailer = SmtpTransport::relay(server)
            .map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?
            .credentials(creds)
            .build();

        mailer.send(&email).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(())
    }
}

impl Default for EmailTool {
    fn default() -> Self { Self::new() }
}
