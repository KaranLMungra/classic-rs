pub mod trifid;
use trifid::Trifid;
pub trait Cipher {
    type Key;
    fn cipher(key: Self::Key) -> Self;
    fn encoder(&self, text: Vec<u8>) -> Vec<u8>;
    fn decoder(&self, cipher: Vec<u8>) -> Vec<u8>;
    fn key(&self) -> Self::Key;
}

pub enum Ciphers {
    Trifid(Trifid),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enc_check() {
        let key = [
            [[b'E', b'P', b'S'], [b'D', b'U', b'C'], [b'V', b'W', b'Y']],
            [[b'M', b'.', b'Z'], [b'L', b'K', b'X'], [b'N', b'B', b'T']],
            [[b'F', b'G', b'O'], [b'R', b'I', b'J'], [b'H', b'A', b'Q']],
        ];
        let trifid = Trifid::cipher(key);
        let text = "DEFEND THE EAST WALL OF THE CASTLE.";
        let text = Vec::from(text.as_bytes());
        let cipher = trifid.encoder(text);
        assert_eq!("SUSEMCPFHTGYQYOXISMLFONCGLBSP".as_bytes(), cipher);
    }
}
