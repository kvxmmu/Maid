use crate::{
    backend::IBackend,
    backends::platform::*,
};

#[derive(Default)]
pub struct AssemblyCompiler {
    backend: Backend,
}

impl AssemblyCompiler {
    pub const fn backend(&self) -> &dyn IBackend {
        &self.backend
    }
}
