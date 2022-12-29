use integral_enum::IntegralEnum;
use snafu::Snafu;

#[derive(IntegralEnum, Snafu)]
#[enum_disable(display)]
pub enum MemOpError {
    #[snafu(display("Invalid location specified: Out of bounds"))]
    OutOfBounds,
}
