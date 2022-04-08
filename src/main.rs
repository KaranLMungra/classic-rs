use ciphers::{*, trifid::Trifid};

fn main() {
    let key = [
        [[b'E', b'P', b'S'], [b'D', b'U', b'C'], [b'V', b'W', b'Y']],
        [[b'M', b'.', b'Z'], [b'L', b'K', b'X'], [b'N', b'B', b'T']],
        [[b'F', b'G', b'O'], [b'R', b'I', b'J'], [b'H', b'A', b'Q']],
    ];
    let trifid = Trifid::cipher(key);
    let text = "DEFEND THE EAST WALL OF THE CASTLE.";
    let text = Vec::from(text.as_bytes());
    let cipher = trifid.encoder(text);
    println!("{}", String::from_utf8(cipher).unwrap());
}
