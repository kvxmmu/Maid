use maid_codec::arm64::decoder::BufferedDecoder;

fn main() {
    let data = include_bytes!("../data/add.bintest");
    let mut decoder = BufferedDecoder::new(data);

    while let Ok((d, instruction)) = decoder.decode_next() {
        println!("{d} {instruction:#?}");
    }
}
