use crate::Cipher;
use std::collections::HashMap;

pub struct Trifid {
    key: [[[u8; 3]; 3]; 3],
}

impl Cipher for Trifid {
    type Key = [[[u8; 3]; 3]; 3];
    fn cipher(key: Self::Key) -> Self {
        Self { key }
    }
    fn encoder(&self, text: Vec<u8>) -> Vec<u8> {
        let mut cipher = Vec::new();
        let code = self.code();
        let mut enc: Vec<Vec<usize>> = Vec::new();
        for i in 0..text.len() {
            if text[i] == b' ' {
                continue;
            }
            let c = code[&text[i]];
            enc.push(Vec::from(c));
        }
        for k in 1..=enc.len() / 5 {
            let sq = &enc[(k - 1) * 5..k * 5];
            let mut j = 0;
            for i in (0..(sq.len() * sq[0].len()) - 2).step_by(3) {
                let (s, r, c) = get_values_5(i, &mut j, &sq);
                cipher.push(self.key[s][r][c]);
            }
        }
        let k = enc.len() / 5;
        let sq = &enc[(k) * 5..k * 5 + (enc.len() % 5)];
        let mut j = 0;
        for i in (0..(sq.len() * sq[0].len()) - 2).step_by(3) {
            let (s, r, c) = get_values_4(i, &mut j, &sq);
            cipher.push(self.key[s][r][c]);
        }
        cipher
    }
    fn decoder(&self, _cipher: Vec<u8>) -> Vec<u8> {
        todo!()
    }
    fn key(&self) -> Self::Key {
        self.key
    }
}

fn get_values_5(i: usize, j: &mut usize, sq: &&[std::vec::Vec<usize>]) -> (usize, usize, usize) {
    match i % 4 {
        0 | 1 | 2 => (sq[i % 4][*j], sq[i % 4 + 1][*j], sq[i % 4 + 2][*j]),
        3 => {
            *j += 1;
            (sq[i % 4][*j - 1], sq[i % 4 + 1][*j - 1], sq[0][*j])
        }
        4 => {
            *j += 1;
            (sq[i % 4][*j - 1], sq[0][*j], sq[1][*j])
        }
        _ => (0, 0, 0),
    }
}
fn get_values_4(i: usize, j: &mut usize, sq: &&[std::vec::Vec<usize>]) -> (usize, usize, usize) {
    match i % 4 {
        0 | 1 => (sq[i % 4][*j], sq[i % 4 + 1][*j], sq[i % 4 + 2][*j]),
        2 => {
            *j += 1;
            (sq[2][*j - 1], sq[3][*j - 1], sq[0][*j])
        }
        3 => {
            *j += 1;
            (sq[i % 4][*j - 1], sq[0][*j], sq[1][*j])
        }
        _ => (0, 0, 0),
    }
}

impl Trifid {
    fn code(&self) -> HashMap<u8, [usize; 3]> {
        let mut s = HashMap::new();
        for sq in 0..self.key.len() {
            for r in 0..self.key[sq].len() {
                for c in 0..self.key[sq][r].len() {
                    s.insert(self.key[sq][r][c], [sq, r, c]);
                }
            }
        }
        s
    }

}
