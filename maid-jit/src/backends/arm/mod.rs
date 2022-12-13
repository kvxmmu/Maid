use crate::backend::IBackend;

pub struct Backend {}

impl Default for Backend {
    fn default() -> Self {
        Self {}
    }
}

impl IBackend for Backend {
    fn name(&self) -> String {
        "aarch64".to_owned()
    }
}
