use crate::backend::IBackend;

#[derive(Default)]
pub struct Backend {}

impl IBackend for Backend {
    fn name(&self) -> String {
        "MIPS".to_owned()
    }
}
