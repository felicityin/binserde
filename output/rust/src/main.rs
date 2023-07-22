use types::hello::Hello;

fn main() {
    let entity = Hello { a: 1, b: 2 };

    let encoded: Vec<u8> = entity.pack().unwrap();
    println!("encoded: {:?}, len: {}", encoded, encoded.len());

    let (decoded, len): (Hello, usize) = Hello::default().unpack(&encoded).unwrap();
    println!("decoded: {:?}, len: {}\n", decoded, len);
}
