use teaql_runtime::UserContext;
use std::future::IntoFuture;
use std::pin::Pin;

#[cfg(feature = "http")]
pub trait ContextHttpExt {
    fn http(&self) -> ContextHttpFacade<'_>;
}

#[cfg(feature = "http")]
impl ContextHttpExt for UserContext {
    fn http(&self) -> ContextHttpFacade<'_> {
        ContextHttpFacade { ctx: self }
    }
}

#[cfg(feature = "http")]
pub struct ContextHttpFacade<'a> {
    ctx: &'a UserContext,
}

#[cfg(feature = "http")]
impl<'a> ContextHttpFacade<'a> {
    pub fn get(self, url: impl Into<String>) -> PendingHttpGet<'a> {
        PendingHttpGet {
            ctx: self.ctx,
            url: url.into(),
        }
    }
}

#[cfg(feature = "http")]
pub struct PendingHttpGet<'a> {
    ctx: &'a UserContext,
    url: String,
}

#[cfg(feature = "http")]
impl<'a> PendingHttpGet<'a> {
    pub fn comment(self, desc: impl Into<String>) -> ExecutableHttpGet<'a> {
        ExecutableHttpGet {
            ctx: self.ctx,
            url: self.url,
            comment: desc.into(),
        }
    }
}

#[cfg(feature = "http")]
pub struct ExecutableHttpGet<'a> {
    ctx: &'a UserContext,
    url: String,
    comment: String,
}

#[cfg(feature = "http")]
impl<'a> IntoFuture for ExecutableHttpGet<'a> {
    type Output = Result<String, String>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let trace_id = self.ctx.user_identifier().unwrap_or("system");
            
            // Note: Since UnifiedLogBuffer methods aren't exported exactly as `log_audit` in UserContext,
            // we use println for the facade proof-of-concept. In a full implementation, this bridges to the EventSink or Logger.
            println!("[AUDIT] [Trace: {}] [HTTP GET] Intent: [{}] URL: {}", trace_id, self.comment, self.url);
            
            let start = std::time::Instant::now();
            let result = reqwest::get(&self.url)
                .await
                .map_err(|e| e.to_string())?
                .text()
                .await
                .map_err(|e| e.to_string());
            
            if result.is_ok() {
                println!("[AUDIT] [Trace: {}] [HTTP GET SUCCESS] Time: {:?}", trace_id, start.elapsed());
            } else {
                println!("[AUDIT] [Trace: {}] [HTTP GET FAILED] Time: {:?}", trace_id, start.elapsed());
            }
            
            result
        })
    }
}
