//! PAC Server Module
//!
//! Provides Proxy Auto-Configuration (PAC) server including:
//! - Dynamic PAC script generation
//! - Rule-based proxy selection
//! - Local HTTP server for PAC file serving

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// PAC (Proxy Auto-Configuration) file server for configuring browser proxies
pub struct PacServer {
    /// Local address to bind to
    bind_addr: std::net::SocketAddr,
    /// PAC file mappings (tab_id -> proxy_port)
    pac_files: Arc<RwLock<HashMap<String, u16>>>,
    /// Running state
    is_running: Arc<RwLock<bool>>,
}

impl PacServer {
    /// Create a new PAC server
    pub fn new(bind_port: u16) -> Result<Self> {
        let bind_addr = format!("127.0.0.1:{}", bind_port)
            .parse()
            .map_err(|e| anyhow!("Invalid bind address: {}", e))?;

        Ok(Self {
            bind_addr,
            pac_files: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(RwLock::new(false)),
        })
    }

    /// Start the PAC server
    pub async fn start(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            return Err(anyhow!("PAC server is already running"));
        }

        let listener = TcpListener::bind(&self.bind_addr)
            .await
            .map_err(|e| anyhow!("Failed to bind to {}: {}", self.bind_addr, e))?;

        info!("PAC server listening on {}", self.bind_addr);
        *is_running = true;
        drop(is_running);

        let pac_files = self.pac_files.clone();
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            while *is_running.read().await {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        debug!("PAC request from {}", addr);
                        let pac_files_clone = pac_files.clone();

                        tokio::spawn(async move {
                            if let Err(e) = Self::handle_pac_request(stream, pac_files_clone).await
                            {
                                error!("Error handling PAC request: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        if *is_running.read().await {
                            error!("Error accepting PAC request: {}", e);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop the PAC server
    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.write().await;
        *is_running = false;

        // Clear all PAC files
        let mut pac_files = self.pac_files.write().await;
        pac_files.clear();

        info!("PAC server stopped");
        Ok(())
    }

    /// Register a PAC file for a tab
    pub async fn register_pac_for_tab(&self, tab_id: &str, proxy_port: u16) -> Result<String> {
        {
            let mut pac_files = self.pac_files.write().await;
            pac_files.insert(tab_id.to_string(), proxy_port);
        }

        let pac_url = format!("http://{}/pac/{}", self.bind_addr, tab_id);
        info!("Registered PAC for tab {} -> {}", tab_id, pac_url);

        Ok(pac_url)
    }

    /// Remove PAC file for a tab
    pub async fn remove_pac_for_tab(&self, tab_id: &str) -> Result<()> {
        {
            let mut pac_files = self.pac_files.write().await;
            pac_files.remove(tab_id);
        }

        info!("Removed PAC for tab {}", tab_id);
        Ok(())
    }

    /// Handle PAC file requests
    async fn handle_pac_request(
        mut stream: TcpStream,
        pac_files: Arc<RwLock<HashMap<String, u16>>>,
    ) -> Result<()> {
        // Read HTTP request
        let mut buffer = vec![0u8; 4096];
        let n = stream.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n]);

        // Parse request to extract tab ID from URL
        let tab_id = Self::extract_tab_id_from_request(&request)?;

        // Get proxy port for this tab
        let proxy_port = {
            let pac_files = pac_files.read().await;
            pac_files.get(&tab_id).copied().unwrap_or(0)
        };

        // Generate PAC file content
        let pac_content = Self::generate_pac_content(proxy_port);

        // Send HTTP response with PAC file
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: application/x-ns-proxy-autoconfig\r\n\
             Content-Length: {}\r\n\
             Access-Control-Allow-Origin: *\r\n\
             \r\n\
             {}",
            pac_content.len(),
            pac_content
        );

        stream.write_all(response.as_bytes()).await?;
        debug!(
            "Served PAC file for tab {} with proxy port {}",
            tab_id, proxy_port
        );

        Ok(())
    }

    /// Extract tab ID from HTTP request URL
    fn extract_tab_id_from_request(request: &str) -> Result<String> {
        let lines: Vec<&str> = request.lines().collect();
        if lines.is_empty() {
            return Err(anyhow!("Empty request"));
        }

        let request_line = lines[0];
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(anyhow!("Invalid HTTP request"));
        }

        let url = parts[1];
        if !url.starts_with("/pac/") {
            return Err(anyhow!("Not a PAC request"));
        }

        let tab_id = url
            .strip_prefix("/pac/")
            .ok_or_else(|| anyhow!("Invalid PAC URL"))?
            .split('/')
            .next()
            .ok_or_else(|| anyhow!("No tab ID in URL"))?
            .to_string();

        Ok(tab_id)
    }

    /// Generate PAC file content
    fn generate_pac_content(proxy_port: u16) -> String {
        if proxy_port == 0 {
            // No proxy - direct connection
            return r#"
function FindProxyForURL(url, host) {
    return "DIRECT";
}
"#
            .to_string();
        }

        format!(
            r#"
function FindProxyForURL(url, host) {{
    // Route all traffic through the local proxy
    if (isInNet(host, "127.0.0.0", "255.0.0.0") ||
        isPlainHostName(host) ||
        dnsDomainIs(host, ".local")) {{
        return "DIRECT";
    }}
    
    return "PROXY 127.0.0.1:{}";
}}
"#,
            proxy_port
        )
    }

    /// Check if the PAC server is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Get the base URL for PAC files
    pub fn get_base_url(&self) -> String {
        format!("http://{}", self.bind_addr)
    }
}

/// Manager for PAC server integration with browser tabs
pub struct PacManager {
    /// PAC server instance
    pac_server: Arc<PacServer>,
    /// Tab to proxy port mappings
    tab_proxies: Arc<RwLock<HashMap<String, u16>>>,
}

impl PacManager {
    /// Create a new PAC manager
    pub fn new(pac_port: u16) -> Result<Self> {
        Ok(Self {
            pac_server: Arc::new(PacServer::new(pac_port)?),
            tab_proxies: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start the PAC manager
    pub async fn start(&self) -> Result<()> {
        self.pac_server.start().await
    }

    /// Stop the PAC manager
    pub async fn stop(&self) -> Result<()> {
        self.pac_server.stop().await?;
        self.tab_proxies.write().await.clear();
        Ok(())
    }

    /// Register a proxy for a tab and return PAC URL
    pub async fn register_proxy_for_tab(&self, tab_id: &str, proxy_port: u16) -> Result<String> {
        {
            let mut tab_proxies = self.tab_proxies.write().await;
            tab_proxies.insert(tab_id.to_string(), proxy_port);
        }

        self.pac_server
            .register_pac_for_tab(tab_id, proxy_port)
            .await
    }

    /// Remove proxy for a tab
    pub async fn remove_proxy_for_tab(&self, tab_id: &str) -> Result<()> {
        {
            let mut tab_proxies = self.tab_proxies.write().await;
            tab_proxies.remove(tab_id);
        }

        self.pac_server.remove_pac_for_tab(tab_id).await
    }

    /// Get PAC URL for a tab
    pub async fn get_pac_url_for_tab(&self, tab_id: &str) -> Option<String> {
        let tab_proxies = self.tab_proxies.read().await;
        if tab_proxies.contains_key(tab_id) {
            Some(format!(
                "http://{}/pac/{}",
                self.pac_server.bind_addr, tab_id
            ))
        } else {
            None
        }
    }

    /// Get all registered tab proxies
    pub async fn get_registered_proxies(&self) -> HashMap<String, u16> {
        self.tab_proxies.read().await.clone()
    }
}
