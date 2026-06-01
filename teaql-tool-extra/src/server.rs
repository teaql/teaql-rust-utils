use teaql_tool_core::{Result, TeaQLToolError};
use tower_http::services::ServeDir;
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct ServerTool;

impl ServerTool {
    pub fn new() -> Self { Self }

    pub fn serve_dir(&self, dir: &str, port: u16) -> Result<()> {
        let rt = Runtime::new().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let addr = format!("0.0.0.0:{}", port);
        
        rt.block_on(async move {
            let app = axum::Router::new().fallback_service(ServeDir::new(dir));
            let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            println!("Static server listening on http://{}", addr);
            axum::serve(listener, app).await.map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
        })
    }
}

impl Default for ServerTool {
    fn default() -> Self { Self::new() }
}
