use crate::backend::{
    BackendInfo,
    BackendStatus,
    IBackend,
};

#[derive(Default)]
pub struct Backend {}

impl IBackend for Backend {
    fn info(&self) -> BackendInfo {
        BackendInfo {
            status: BackendStatus::NotImplemented,
            name: "x86_64".to_owned(),
        }
    }
}
