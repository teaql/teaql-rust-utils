use crate::macros::define_context_facade;

use teaql_tool_extra::cache::CacheTool;

define_context_facade!("extra", cache, ContextCacheExt, ContextCacheFacade);

// NOTE: CacheTool is stateful — it wraps a `moka::sync::Cache` with a 1-hour TTL.
// Each call to these facade methods creates a NEW CacheTool instance with its own
// independent cache, so data will NOT persist across calls. For production use,
// store a CacheTool instance externally and call its methods directly.
#[cfg(feature = "extra")]
impl<'a> ContextCacheFacade<'a> {
    /// Creates a new CacheTool instance.
    /// NOTE: The returned instance has its own independent cache store.
    pub fn create(&self) -> CacheTool {
        CacheTool::new()
    }
}
