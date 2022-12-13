use {
    maid_vm::codec::arm64::decoder::BufferedDecoder,
    std::{
        fs,
        io,
        path::PathBuf,
    },
};

pub fn enumerate_asm(path: PathBuf) -> io::Result<()> {
    let contents = fs::read(path)?;
    let mut decoder = BufferedDecoder::new(&contents);

    while let Ok(instruction) = decoder.decode_next() {
        println!("-- {:?}", instruction);
    }

    Ok(())
}
