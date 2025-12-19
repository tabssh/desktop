//! SFTP file transfer management

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TransferManager {
    active_transfers: Arc<Mutex<Vec<Transfer>>>,
}

#[derive(Debug, Clone)]
pub struct Transfer {
    pub id: uuid::Uuid,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub direction: TransferDirection,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub status: TransferStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransferDirection {
    Upload,
    Download,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransferStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl TransferManager {
    pub fn new() -> Self {
        Self {
            active_transfers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub async fn add_upload(&self, local_path: PathBuf, remote_path: PathBuf, size: u64) -> uuid::Uuid {
        let transfer = Transfer {
            id: uuid::Uuid::new_v4(),
            source: local_path,
            destination: remote_path,
            direction: TransferDirection::Upload,
            total_bytes: size,
            transferred_bytes: 0,
            status: TransferStatus::Pending,
            error: None,
        };
        
        let id = transfer.id;
        let mut transfers = self.active_transfers.lock().await;
        transfers.push(transfer);
        id
    }
    
    pub async fn add_download(&self, remote_path: PathBuf, local_path: PathBuf, size: u64) -> uuid::Uuid {
        let transfer = Transfer {
            id: uuid::Uuid::new_v4(),
            source: remote_path,
            destination: local_path,
            direction: TransferDirection::Download,
            total_bytes: size,
            transferred_bytes: 0,
            status: TransferStatus::Pending,
            error: None,
        };
        
        let id = transfer.id;
        let mut transfers = self.active_transfers.lock().await;
        transfers.push(transfer);
        id
    }
    
    pub async fn update_progress(&self, id: uuid::Uuid, transferred: u64) {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(transfer) = transfers.iter_mut().find(|t| t.id == id) {
            transfer.transferred_bytes = transferred;
            transfer.status = TransferStatus::InProgress;
        }
    }
    
    pub async fn complete_transfer(&self, id: uuid::Uuid) {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(transfer) = transfers.iter_mut().find(|t| t.id == id) {
            transfer.status = TransferStatus::Completed;
            transfer.transferred_bytes = transfer.total_bytes;
        }
    }
    
    pub async fn fail_transfer(&self, id: uuid::Uuid, error: String) {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(transfer) = transfers.iter_mut().find(|t| t.id == id) {
            transfer.status = TransferStatus::Failed;
            transfer.error = Some(error);
        }
    }
    
    pub async fn cancel_transfer(&self, id: uuid::Uuid) {
        let mut transfers = self.active_transfers.lock().await;
        if let Some(transfer) = transfers.iter_mut().find(|t| t.id == id) {
            transfer.status = TransferStatus::Cancelled;
        }
    }
    
    pub async fn get_active_transfers(&self) -> Vec<Transfer> {
        let transfers = self.active_transfers.lock().await;
        transfers.clone()
    }
    
    pub async fn clear_completed(&self) {
        let mut transfers = self.active_transfers.lock().await;
        transfers.retain(|t| !matches!(t.status,TransferStatus::Completed));
    }
    
    pub async fn get_transfer(&self, id: uuid::Uuid) -> Option<Transfer> {
        let transfers = self.active_transfers.lock().await;
        transfers.iter().find(|t| t.id == id).cloned()
    }
}

impl Default for TransferManager {
    fn default() -> Self {
        Self::new()
    }
}
