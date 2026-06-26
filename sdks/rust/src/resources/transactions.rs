use crate::client::SynapseClient;
use crate::error::SynapseError;
use crate::models::Transaction;

pub struct Transactions<'a> {
    pub(crate) client: &'a SynapseClient,
}

impl<'a> Transactions<'a> {
    /// Fetch a single transaction by its UUID.
    ///
    /// Returns [`SynapseError::NotFound`] when the ID does not exist so callers
    /// can distinguish a missing record from other failure modes without
    /// inspecting raw HTTP status codes.
    ///
    /// # Errors
    /// - [`SynapseError::NotFound`] – no transaction with this ID exists (HTTP 404).
    /// - [`SynapseError::Api`] – server returned another non-success status.
    /// - [`SynapseError::Http`] – network error.
    /// - [`SynapseError::Decode`] – response body is not valid JSON.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use synapse_sdk::{SynapseClient, SynapseError};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SynapseClient::new("https://api.example.com", "your-api-key");
    ///
    /// match client.transactions().get("550e8400-e29b-41d4-a716-446655440000").await {
    ///     Ok(tx) => println!("status: {}", tx.status),
    ///     Err(SynapseError::NotFound(msg)) => eprintln!("not found: {}", msg),
    ///     Err(e) => eprintln!("error: {}", e),
    /// }
    /// # }
    /// ```
    pub async fn get(&self, id: &str) -> Result<Transaction, SynapseError> {
        let path = format!("/transactions/{}", id);
        match self.client.get::<Transaction>(&path).await {
            Err(SynapseError::Api { status: 404, message }) => {
                Err(SynapseError::NotFound(message))
            }
            other => other,
        }
    }
}
