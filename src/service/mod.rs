use axum::http::{HeaderMap, HeaderValue};
use rmcp::{handler::server::tool::{Parameters, ToolRouter}, model::{CallToolResult, Content, ServerCapabilities}, tool, tool_handler, tool_router, ServerHandler};
use serde::Serialize;

use crate::{error::{McpError}, model::{KtpExtractRequest, KtpExtractResponse}};

pub mod config;

const VERIHUBS_BASE_URL: &str = "https://api.verihubs.com";

pub struct VerihubsService {
    client: reqwest::Client,
    tool_router: ToolRouter<VerihubsService>
}

#[tool_router]
impl VerihubsService {
    #[allow(dead_code)]
    pub fn new(config: config::VerihubsConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("App-ID", config.app_id.parse().expect("Failed to parse app id"));
        headers.insert("API-Key", config.api_key.parse().expect("Failed to parse api key"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build http client");

        Self {
            client: client,
            tool_router: Self::tool_router()
        }
    }

    #[tool(description = "Get id card (KTP) data from image")]
    pub async fn extract_ktp_data(
        &self, 
        Parameters(KtpExtractRequest{ image}): Parameters<KtpExtractRequest> 
    ) -> std::result::Result<CallToolResult, rmcp::Error> {
        let data = KtpExtractRequest {
            image
        };
        let response = self.make_request(data).await;
        let result = match response {
            Ok(v) => {
                let jsonstring = serde_json::to_string(&v)
                    .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))?;

                CallToolResult::success(vec![
                    Content::text(jsonstring)
                ])
            }, 
            Err(e) => {
                tracing::error!("Error request to Verihubs API: {}", e);
                CallToolResult::error(vec![
                    Content::text("Failed to get result".to_string())
                ])
            }
        };

        
        Ok(result)
    }

    pub async fn make_request<T: Serialize>(&self, data: T) -> Result<KtpExtractResponse, McpError>{
        let post = self.client.post(format!("{VERIHUBS_BASE_URL}/v2/ktp/extract"))
            .json(&data)
            .send()
            .await?;
        let text = post.text().await?;
        dbg!(&text);

        let json: KtpExtractResponse = serde_json::from_str(&text)?;

        Ok(json)
    }



}

#[tool_handler]
impl ServerHandler for VerihubsService {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            instructions: Some("A verification service and AI services".to_string()),
            ..Default::default()
        }
    }
}