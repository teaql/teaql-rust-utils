use teaql_tool_core::{Result, TeaQLToolError};
use axum::{extract::Request, response::IntoResponse, Router};
use reqwest::Client;
use tokio::runtime::Runtime;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ProxyTool;

impl ProxyTool {
    pub fn new() -> Self { Self }

    pub fn start(&self, listen_port: u16, target_url: &str) -> Result<()> {
        let rt = Runtime::new().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let target = Arc::new(target_url.trim_end_matches('/').to_string());
        
        rt.block_on(async move {
            let client = Client::new();
            
            let app = Router::new().fallback(move |req: Request| {
                let client = client.clone();
                let target = target.clone();
                
                async move {
                    let path_query = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("");
                    let url = format!("{}{}", target, path_query);
                    
                    let mut proxy_req = client.request(req.method().clone(), &url);
                    for (k, v) in req.headers() {
                        if k != reqwest::header::HOST {
                            proxy_req = proxy_req.header(k.clone(), v.clone());
                        }
                    }
                    
                    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap_or_default();
                    proxy_req = proxy_req.body(body_bytes);
                    
                    match proxy_req.send().await {
                        Ok(resp) => {
                            let mut builder = axum::http::Response::builder().status(resp.status());
                            for (k, v) in resp.headers() {
                                builder = builder.header(k.clone(), v.clone());
                            }
                            let stream = resp.bytes_stream();
                            let body = axum::body::Body::from_stream(stream);
                            builder.body(body).unwrap_or_else(|_| axum::http::Response::new(axum::body::Body::empty()))
                        }
                        Err(e) => {
                            axum::http::Response::builder()
                                .status(502)
                                .body(axum::body::Body::from(format!("Bad Gateway: {}", e)))
                                .unwrap()
                        }
                    }
                }
            });

            let addr = format!("0.0.0.0:{}", listen_port);
            let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            println!("Proxy server listening on http://{} -> {}", addr, target_url);
            axum::serve(listener, app).await.map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
        })
    }
}

impl Default for ProxyTool {
    fn default() -> Self { Self::new() }
}
