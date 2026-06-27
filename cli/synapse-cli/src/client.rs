use anyhow::Result;
use reqwest::Client;

pub struct SynapseCliClient {
    client: Client,
    base_url: String,
}

impl SynapseCliClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).send().await?;
        response.json().await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn get_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query_params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);

        for (key, value) in query_params {
            req = req.query(&[(key, value)]);
        }

        let response = req.send().await?;
        response.json().await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn get_bytes(
        &self,
        path: &str,
        query_params: &[(&str, &str)],
    ) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.get(&url);

        for (key, value) in query_params {
            req = req.query(&[(key, value)]);
        }

        let response = req.send().await?;
        response.bytes().await.map(|b| b.to_vec()).map_err(|e| anyhow::anyhow!(e))
    }
}
