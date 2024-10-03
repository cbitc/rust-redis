use std::io::{Cursor, Read, Write};

use bytes::{Buf, BytesMut};

fn main() {
    let mut bytes: &[u8] = b"1\n2321\nsfsas\n\tasd\n";

    let mut chunk: [u8; 4] = [0; 4];
    let mut data = Vec::new();

    loop {
        if let Some(v) = check(&mut data){
            println!("{}",String::from_utf8(v).unwrap());
        }
        let n = bytes.read(&mut chunk[..]).unwrap();
        if n == 0 {
            break;
        }
        data.write_all(&chunk[..n]).unwrap();
    }

}

fn check(data: &mut Vec<u8>) -> Option<Vec<u8>> {
    let mut i = 0;
    while i < data.len() && data[i] != b'\n' {
        i += 1;
    }
    if i < data.len() {
        let result = (&data[..i]).to_vec();
        let re_len = data.len() - i - 1;
        for j in 0..re_len {
            data[j] = data[j + i + 1];
        }
        data.resize(re_len, 0);
        return Some(result);
    }
    None
}
