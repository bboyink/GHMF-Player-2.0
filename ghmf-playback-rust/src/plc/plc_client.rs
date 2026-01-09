use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{Context, Result};

/// PLC connection status
#[derive(Debug, Clone, PartialEq)]
pub enum PlcStatus {
    Disabled,
    Disconnected,
    Connected,
    Reconnecting,
}

/// PLC Client for sending water commands to a Programmable Logic Controller
/// Matches the behavior of the C# PLCComms.cs class
pub struct PlcClient {
    enabled: bool,
    ip_address: String,
    port: u16,
    stream: Arc<Mutex<Option<TcpStream>>>,
    command_queue: Arc<Mutex<Vec<String>>>,
    status: Arc<Mutex<PlcStatus>>,
    reconnecting: Arc<Mutex<bool>>,
}

impl PlcClient {
    /// Create a new PLC client
    pub fn new(enabled: bool, ip_address: String, port: u16) -> Self {
        let status = if enabled {
            PlcStatus::Disconnected
        } else {
            PlcStatus::Disabled
        };

        Self {
            enabled,
            ip_address,
            port,
            stream: Arc::new(Mutex::new(None)),
            command_queue: Arc::new(Mutex::new(Vec::new())),
            status: Arc::new(Mutex::new(status)),
            reconnecting: Arc::new(Mutex::new(false)),
        }
    }

    /// Get the current connection status
    pub async fn status(&self) -> PlcStatus {
        self.status.lock().await.clone()
    }

    /// Check if connected (or disabled, which counts as "ok")
    pub async fn is_connected(&self) -> bool {
        if !self.enabled {
            return true; // Disabled counts as "connected" for logic purposes
        }
        
        let status = self.status.lock().await;
        *status == PlcStatus::Connected
    }

    /// Connect to the PLC with a timeout
    pub async fn connect(&self, timeout_ms: u64) -> Result<()> {
        if !self.enabled {
            tracing::debug!("PLC is disabled, skipping connection");
            return Ok(());
        }

        // Disconnect if already connected
        self.disconnect().await;

        tracing::info!("Connecting to PLC at {}:{}", self.ip_address, self.port);
        
        let addr = format!("{}:{}", self.ip_address, self.port);
        
        // Connect with timeout
        let connect_future = TcpStream::connect(&addr);
        let stream = tokio::time::timeout(
            tokio::time::Duration::from_millis(timeout_ms),
            connect_future
        )
        .await
        .context("PLC connection timed out")?
        .context("Failed to connect to PLC")?;

        *self.stream.lock().await = Some(stream);
        *self.status.lock().await = PlcStatus::Connected;
        *self.reconnecting.lock().await = false;

        tracing::info!("Successfully connected to PLC");
        Ok(())
    }

    /// Disconnect from the PLC
    pub async fn disconnect(&self) {
        let mut stream = self.stream.lock().await;
        if let Some(mut s) = stream.take() {
            let _ = s.shutdown().await;
            tracing::debug!("Disconnected from PLC");
        }
        *self.status.lock().await = PlcStatus::Disconnected;
    }

    /// Add a command to the queue (will be sent in batch)
    /// Synchronous version that doesn't block
    pub fn queue_command_sync(&self, command: String) {
        if command.trim().is_empty() {
            return;
        }

        // Try to lock without blocking
        if let Ok(mut queue) = self.command_queue.try_lock() {
            queue.push(command);
            tracing::trace!("Added command to PLC queue, queue size: {}", queue.len());
        } else {
            tracing::warn!("Failed to queue PLC command - queue locked");
        }
    }

    /// Add a command to the queue (will be sent in batch)
    pub async fn add_to_queue(&self, command: String) {
        if command.trim().is_empty() {
            return;
        }

        let mut queue = self.command_queue.lock().await;
        queue.push(command);
        tracing::trace!("Added command to PLC queue, queue size: {}", queue.len());
    }

    /// Send all queued commands in a single batch
    /// Returns the number of commands sent
    pub async fn send_queue(&self) -> Result<usize> {
        let mut queue = self.command_queue.lock().await;
        let count = queue.len();

        if count == 0 {
            return Ok(0);
        }

        // Join all commands with spaces and add line terminator
        let message = format!("{}\r\n", queue.join(" "));
        
        // Clear queue before sending (so we don't resend on failure)
        queue.clear();
        drop(queue); // Release lock before sending

        // Send the batch
        self.send(&message).await?;

        Ok(count)
    }

    /// Send a string to the PLC
    async fn send(&self, message: &str) -> Result<()> {
        if !self.enabled {
            tracing::debug!("Would have sent to PLC: {}", message.trim());
            return Ok(());
        }

        let mut stream_guard = self.stream.lock().await;
        
        if let Some(stream) = stream_guard.as_mut() {
            tracing::debug!("Sending to PLC: {}", message.trim());
            
            match stream.write_all(message.as_bytes()).await {
                Ok(_) => {
                    // Ensure data is flushed
                    stream.flush().await.context("Failed to flush PLC stream")?;
                    Ok(())
                }
                Err(e) => {
                    tracing::warn!("PLC disconnected during send: {}. Will attempt reconnect.", e);
                    *stream_guard = None;
                    *self.status.lock().await = PlcStatus::Disconnected;
                    
                    // Trigger reconnect attempt
                    drop(stream_guard); // Release lock
                    self.attempt_reconnect().await;
                    
                    Err(anyhow::anyhow!("PLC send failed: {}", e))
                }
            }
        } else {
            // Not connected, try to reconnect
            drop(stream_guard); // Release lock
            self.attempt_reconnect().await;
            Err(anyhow::anyhow!("PLC not connected"))
        }
    }

    /// Attempt to reconnect to the PLC (non-blocking)
    async fn attempt_reconnect(&self) {
        let mut reconnecting = self.reconnecting.lock().await;
        if *reconnecting {
            return; // Already reconnecting
        }
        *reconnecting = true;
        *self.status.lock().await = PlcStatus::Reconnecting;
        drop(reconnecting);

        tracing::info!("Attempting to reconnect to PLC");

        // Spawn reconnect attempt (non-blocking)
        let self_clone = Self {
            enabled: self.enabled,
            ip_address: self.ip_address.clone(),
            port: self.port,
            stream: Arc::clone(&self.stream),
            command_queue: Arc::clone(&self.command_queue),
            status: Arc::clone(&self.status),
            reconnecting: Arc::clone(&self.reconnecting),
        };

        tokio::spawn(async move {
            match self_clone.connect(1000).await {
                Ok(_) => tracing::info!("PLC reconnection successful"),
                Err(e) => {
                    tracing::warn!("PLC reconnection failed: {}", e);
                    *self_clone.reconnecting.lock().await = false;
                }
            }
        });
    }

    /// Get the current queue size
    pub async fn queue_size(&self) -> usize {
        self.command_queue.lock().await.len()
    }

    /// Clear the command queue (useful for stopping/resetting)
    pub async fn clear_queue(&self) {
        self.command_queue.lock().await.clear();
        tracing::debug!("PLC command queue cleared");
    }
}

impl Drop for PlcClient {
    fn drop(&mut self) {
        // Note: Can't call async disconnect in Drop, but the TcpStream will close automatically
        tracing::debug!("PLC client dropped");
    }
}
