use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum BackendStatus {
    #[display(fmt = "fully implemented")]
    FullyImplemented,

    #[display(
        fmt = "partially implemented, some functionality could not work"
    )]
    PartiallyImplemented,

    #[display(fmt = "not implemented")]
    NotImplemented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendInfo {
    pub status: BackendStatus,
    pub name: String,
}

pub trait IBackend {
    fn info(&self) -> BackendInfo;
}

impl BackendStatus {
    pub fn assert_implemented(self) {
        if self == Self::NotImplemented {
            panic!("Backend is not implemented");
        }
    }
}
