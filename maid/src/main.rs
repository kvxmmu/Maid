use maid_codec::arm64::decoder::*;

fn main() {
    let buffer = include_bytes!("../../data/add.bintest");
    let mut decoder = BufferedDecoder::new(buffer);

    while let Ok(insn) = decoder.decode_next() {
        dbg!(insn);
    }
}
