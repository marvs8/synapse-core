use crate::error::SynapseError;
use crate::resources::transactions::Transactions;

/// HTTP client for the Synapse Core API.
///
/// Create one instance per application and reuse it — it holds a connection pool
/// internally.
pub struct SynapseClient {
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) http: reqwest::Client,
}

impl SynapseClient {
    /// Build a new client.
    ///
    /// * `base_url` – root URL of the API, e.g. `"https://api.example.com"`.
    /// * `api_key`  – tenant API key sent as `X-API-Key` on every request.
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            http: reqwest::Client::new(),
        }
    }

    /// Access transaction-related API methods.
    pub fn transactions(&self) -> Transactions<'_> {
        Transactions { client: self }
    }

    /// Send an authenticated GET and decode the JSON response.
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T, SynapseError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            Ok(serde_json::from_str(&body)?)
        } else {
            Err(SynapseError::Api {
                status: status.as_u16(),
                message: body,
            })
        }
    }

    /// Send an authenticated GET with query parameters and decode the JSON response.
    pub(crate) async fn get_query<T>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T, SynapseError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .query(params)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            Ok(serde_json::from_str(&body)?)
        } else {
            Err(SynapseError::Api {
                status: status.as_u16(),
                message: body,
            })
        }
    }
}
