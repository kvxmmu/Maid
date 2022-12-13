use crate::backend::{
    BackendInfo,
    BackendStatus,
    IBackend,
};

pub struct Backend {}

impl Default for Backend {
    fn default() -> Self {
        Self {}
    }
}

impl IBackend for Backend {
    fn info(&self) -> BackendInfo {
        BackendInfo {
            status: BackendStatus::NotImplemented,
            name: "aarch64".to_owned(),
        }
    }
}
