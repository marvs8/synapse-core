use crate::client::SynapseClient;
use crate::error::SynapseError;
use crate::models::{ListParams, Settlement, SettlementList};

pub struct Settlements<'a> {
    pub(crate) client: &'a SynapseClient,
}

impl<'a> Settlements<'a> {
    /// Fetch a single settlement by its UUID.
    pub async fn get(&self, id: &str) -> Result<Settlement, SynapseError> {
        let path = format!("/settlements/{}", id);
        self.client.get::<Settlement>(&path).await
    }

    /// List settlements with cursor-based pagination.
    pub async fn list(&self, params: ListParams) -> Result<SettlementList, SynapseError> {
        let mut path = format!("/settlements?limit={}&direction=forward", params.limit.unwrap_or(10));
        if let Some(cursor) = &params.cursor {
            path.push_str(&format!("&cursor={}", cursor));
        }
        if let Some(from_date) = &params.from_date {
            path.push_str(&format!("&from_date={}", from_date));
        }
        self.client.get::<SettlementList>(&path).await
    }
}
