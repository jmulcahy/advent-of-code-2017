use std::ops::BitXor;

fn process_once(slice: &[u8], pos: usize, length: u8) -> Vec<u8> {
    let mut slice_copy = slice.to_vec();
    ((pos)..(pos + length as usize)).rev()
        .map(|i| slice[i % slice.len()])
        .collect::<Vec<u8>>().iter()
        .zip(pos..(pos + length as usize))
        .for_each(|(&x, i)| slice_copy[i % slice.len()] = x);
    slice_copy
}

pub fn process1(lengths: &[u8], size: u16) -> u16 {
    let vec = (0..size).map(|x| x as u8).collect::<Vec<u8>>();
    let (result, _, _) = shuffle(&vec, &lengths, 0, 0);
    result[0] as u16 * result[1] as u16
}

pub fn parse_input1(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    input.trim().split(',')
        .map(|num| num.parse()).collect()
}

pub fn parse_input2(input: &str) -> Vec<u8> {
    input.trim().as_bytes().to_vec()
}

fn shuffle(data: &[u8], lengths: &[u8], pos: usize, jump: usize)
    -> (Vec<u8>, usize, usize) {
    let mut vec = data.to_vec();
    let mut pos_copy = pos;
    let mut jump_copy = jump;
    for &length in lengths.iter() {
        vec = process_once(&vec, pos_copy, length);
        pos_copy = (length as usize + jump_copy + pos_copy) % data.len();
        jump_copy += 1;
    }
    (vec, pos_copy, jump_copy)
}

fn dense_hash(sparse_hash: &[u8]) -> u8 {
    sparse_hash.iter().fold(0, BitXor::bitxor)
}

fn hex(hash: &[u8]) -> String {
    hash.iter().map(|h| format!("{:02x}", h)).collect()
}

pub fn process2(lengths: &[u8]) -> String {
    let extra_lengths = [17, 31, 73, 47, 23];
    let mut vec = (0..256).map(|x| x as u8).collect::<Vec<u8>>();
    let mut pos = 0;
    let mut jump = 0;
    for _ in 0..64 {
        let first = shuffle(&vec, &lengths, pos, jump);
        let second = shuffle(&first.0, &extra_lengths, first.1, first.2);
        vec = second.0; pos = second.1; jump = second.2;
    }

    let hash = (0..16)
        .map(|i| dense_hash(&vec[(16 * i)..(16 * (i + 1))])).collect::<Vec<u8>>();
    hex(&hash)
}

#[cfg(test)]
mod tests;
