use crate::backend::IBackend;

#[derive(Default)]
pub struct Backend {}

impl IBackend for Backend {
    fn name(&self) -> String {
        "x86_64".to_owned()
    }
}
