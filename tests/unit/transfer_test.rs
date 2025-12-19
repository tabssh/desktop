//! Transfer manager tests

use tabssh::sftp::transfer::{TransferManager, TransferDirection, TransferStatus};
use std::path::PathBuf;

#[tokio::test]
async fn test_add_upload() {
    let manager = TransferManager::new();
    
    let id = manager.add_upload(
        PathBuf::from("/local/file.txt"),
        PathBuf::from("/remote/file.txt"),
        1024
    ).await;
    
    let transfer = manager.get_transfer(id).await;
    assert!(transfer.is_some());
    
    let transfer = transfer.unwrap();
    assert_eq!(transfer.direction,TransferDirection::Upload);
    assert_eq!(transfer.total_bytes,1024);
    assert_eq!(transfer.status,TransferStatus::Pending);
}

#[tokio::test]
async fn test_update_progress() {
    let manager = TransferManager::new();
    
    let id = manager.add_download(
        PathBuf::from("/remote/file.txt"),
        PathBuf::from("/local/file.txt"),
        2048
    ).await;
    
    manager.update_progress(id, 512).await;
    
    let transfer = manager.get_transfer(id).await.unwrap();
    assert_eq!(transfer.transferred_bytes,512);
    assert_eq!(transfer.status,TransferStatus::InProgress);
}

#[tokio::test]
async fn test_complete_transfer() {
    let manager = TransferManager::new();
    
    let id = manager.add_upload(
        PathBuf::from("/local/file.txt"),
        PathBuf::from("/remote/file.txt"),
        1024
    ).await;
    
    manager.complete_transfer(id).await;
    
    let transfer = manager.get_transfer(id).await.unwrap();
    assert_eq!(transfer.status,TransferStatus::Completed);
    assert_eq!(transfer.transferred_bytes,transfer.total_bytes);
}
