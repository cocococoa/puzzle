use itertools::Itertools;
use std::io::{self, BufRead};
use std::time::Instant;

// 16x16までしか対応しない
// 実行時間的に10x10が限界か？
// mat は col-major
// u64 = 4 bit * 16
struct Latin {
    size: u8,
    mat: [u64; 16],
}
impl Latin {
    fn new(size: u8) -> Self {
        assert!(size < 16);

        Latin {
            size: size,
            mat: [0u64; 16],
        }
    }
    fn from_vec(vec: Vec<Vec<u8>>) -> Self {
        let mut latin = Latin::new(vec.len() as u8);
        for i in 0..latin.size() {
            for j in 0..latin.size() {
                latin.set(i, j, vec[i as usize][j as usize]);
            }
        }

        latin
    }
    fn size(&self) -> u8 {
        self.size
    }
    fn shift_and_mask(x: u64, y: u8) -> u8 {
        let mask = 0xfu64;
        ((x >> y) & mask) as u8
    }
    fn set(&mut self, i: u8, j: u8, v: u8) {
        assert!(i < self.size);
        assert!(j < self.size);

        let mask = 0xfu64 << (i << 2);
        let mask = !mask;
        self.mat[j as usize] &= mask;

        let v = (v as u64) << (i << 2);
        self.mat[j as usize] |= v;
    }
    fn get(&self, i: u8, j: u8) -> u8 {
        assert!(i < self.size);
        assert!(j < self.size);

        Self::shift_and_mask(self.mat[j as usize], i << 2)
    }
}

#[derive(Clone, Copy)]
struct Transversal(u64);
impl Transversal {
    #[allow(dead_code)]
    fn new() -> Self {
        Transversal(0)
    }
    fn from_vec(vec: Vec<u8>) -> Self {
        let mut t = 0u64;
        for x in vec.into_iter().rev() {
            t <<= 1 << 2;
            t |= x as u64;
        }
        Transversal(t)
    }
    fn shift_and_mask(x: u64, y: u8) -> u8 {
        let mask = 0xfu64;
        ((x >> y) & mask) as u8
    }
    #[allow(dead_code)]
    fn set(&mut self, j: u8, v: u8) {
        let mask = 0xfu64 << (j << 2);
        let mask = !mask;
        self.0 &= mask;

        let v = (v as u64) << (j << 2);
        self.0 |= v;
    }
    fn get(&self, j: u8) -> u8 {
        Self::shift_and_mask(self.0, j << 2)
    }
}

fn get_transversals(latin: &Latin) -> Vec<Vec<Transversal>> {
    let mut ret: Vec<Vec<Transversal>> = vec![vec![]; latin.size() as usize];

    for perm in (0..latin.size()).permutations(latin.size() as usize) {
        // check whether this permutation is transversal
        let mut check = [0u8; 16];
        for j in 0..latin.size() {
            let v = latin.get(perm[j as usize], j);
            check[v as usize] = 1;
        }
        let mut ok = true;
        for x in 0..latin.size() {
            if check[x as usize] != 1 {
                ok = false;
            }
        }

        if ok {
            let key = perm[0];
            let perm = Transversal::from_vec(perm);
            ret[key as usize].push(perm);
        }
    }

    ret
}
#[allow(dead_code)]
fn pretty_print_transversal(latin: &Latin, transversal: Transversal) {
    for i in 0..latin.size() {
        for j in 0..latin.size() {
            if transversal.get(j) == i {
                print!("{} ", latin.get(i, j));
            } else {
                print!("● ");
            }
        }
        print!("\n")
    }
}
fn pretty_print_latin(latin: &Latin) {
    for i in 0..latin.size() {
        print!("\t");
        for j in 0..latin.size() {
            print!("{} ", latin.get(i, j));
        }
        print!("\n")
    }
}
fn dfs(
    transversal_map: &Vec<Vec<Transversal>>,
    state: &mut Vec<Transversal>,
    size: u8,
    search: u8,
    answers: &mut Vec<Vec<Transversal>>,
) {
    debug_assert_eq!(state.len(), search as usize);

    let transversals = &transversal_map[search as usize];

    let mut sum = [[0u8; 16]; 16];
    for t in state.iter() {
        for j in 0..size {
            // some transversal uses (t.get(j), j)
            sum[j as usize][t.get(j) as usize] = 1;
        }
    }
    let sum = sum;

    for t in transversals.iter() {
        // state に t を追加して矛盾が生じないかチェックする
        let mut ok = true;
        for j in 0..size {
            if sum[j as usize][t.get(j) as usize] == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            state.push(*t);
            if search + 1 == size {
                answers.push(state.clone());
            } else {
                dfs(transversal_map, state, size, search + 1, answers);
            }
            state.pop();
        }
    }
}
fn search_orthogonal_latin(latin: &Latin, verbose: bool) -> Vec<Latin> {
    let start = Instant::now();
    let transversal_map = get_transversals(latin);

    if verbose {
        let mut sum = 0;
        print!("Transversals: \n\t");
        for i in 0..latin.size() {
            let transversals = &transversal_map[i as usize];
            sum += transversals.len();
            print!("{} ", transversals.len());
        }
        println!("\nTotal transversals: \n\t{}", sum);
    }

    let mut state = vec![];
    let mut answers = vec![];
    state.reserve(100);
    answers.reserve(100);
    dfs(&transversal_map, &mut state, latin.size(), 0, &mut answers);

    let mut ret = vec![];
    for answer in answers.into_iter() {
        let mut x = Latin::new(latin.size());
        for i in 0..latin.size() {
            for j in 0..latin.size() {
                x.set(answer[i as usize].get(j), j, i);
            }
        }
        ret.push(x);
    }

    let end = start.elapsed();
    if verbose {
        println!(
            "Search time: \n\t{}.{:03} [sec]",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }

    ret
}

fn main() {
    let stdio = io::stdin();
    let mut input = stdio.lock();
    let mut line = String::new();
    let mut matrix = vec![];
    while input.read_line(&mut line).unwrap() > 0 {
        matrix.push(line.as_bytes().iter().map(|x| *x).collect::<Vec<u8>>());
        line.clear();
    }

    println!("Input latin square: ");
    for i in 0..10 {
        print!("\t");
        for j in 0..10 {
            matrix[i][j] -= 0x30;
            print!("{} ", matrix[i][j]);
        }
        println!("");
    }

    let latin = Latin::from_vec(matrix);
    let orthogonals = search_orthogonal_latin(&latin, true);
    println!(
        "The number of orthogonal latin squares: \n\t{}",
        orthogonals.len()
    );
    println!("Orthogonal latin squares: ");
    for i in 0..orthogonals.len() {
        if i > 9 {
            println!("以下略");
            break;
        }
        println!("#{}:", i);
        pretty_print_latin(&orthogonals[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latin() {
        let mut latin = Latin::new(3);
        // latin:
        //     0 1 2
        //     1 2 0
        //     2 0 1

        latin.set(0, 0, 0);
        latin.set(0, 1, 1);
        latin.set(0, 2, 2);
        latin.set(1, 0, 1);
        latin.set(1, 1, 2);
        latin.set(1, 2, 0);
        latin.set(2, 0, 2);
        latin.set(2, 1, 0);
        latin.set(2, 2, 1);

        assert_eq!(0, latin.get(0, 0));
        assert_eq!(1, latin.get(0, 1));
        assert_eq!(2, latin.get(0, 2));
        assert_eq!(1, latin.get(1, 0));
        assert_eq!(2, latin.get(1, 1));
        assert_eq!(0, latin.get(1, 2));
        assert_eq!(2, latin.get(2, 0));
        assert_eq!(0, latin.get(2, 1));
        assert_eq!(1, latin.get(2, 2));

        let ts = get_transversals(&latin);
        for i in 0..latin.size() {
            println!("#{}", i);
            pretty_print_transversal(&latin, ts[i as usize][0]);
        }
        assert_eq!(3, ts.len());
    }
}
