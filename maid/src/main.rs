use {
    anyhow::Result,
    maid::{
        actions::enumerate::enumerate_asm,
        args::*,
    },
};

fn main() -> Result<()> {
    let args = CliArgs::from_args();

    match args.sub {
        CliSub::Enumerate { file } => {
            enumerate_asm(file)?;
        }
    }

    Ok(())
}
