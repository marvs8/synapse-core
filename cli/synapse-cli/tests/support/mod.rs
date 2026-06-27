use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MockServer {
    pub base_url: String,
    listener: tokio::net::TcpListener,
}

impl MockServer {
    pub async fn start() -> anyhow::Result<Self> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let base_url = format!("http://{}", addr);

        Ok(Self {
            base_url,
            listener,
        })
    }

    pub async fn handle_request<F>(&self, handler: F) -> anyhow::Result<()>
    where
        F: Fn(&str, &str) -> String,
    {
        if let Ok((socket, _)) = self.listener.accept().await {
            let mut reader = std::io::BufReader::new(&socket);
            let mut request = String::new();
            use std::io::BufRead;
            if reader.read_line(&mut request).is_ok() {
                let parts: Vec<&str> = request.split_whitespace().collect();
                if parts.len() >= 2 {
                    let method = parts[0];
                    let path = parts[1];
                    let response = handler(method, path);
                    let _ = std::io::Write::write_all(&mut (&socket), response.as_bytes());
                }
            }
        }
        Ok(())
    }
}

pub async fn spawn_mock_server(
    routes: Arc<Mutex<Vec<(String, String, String)>>>,
) -> anyhow::Result<MockServer> {
    let server = MockServer::start().await?;
    Ok(server)
}
