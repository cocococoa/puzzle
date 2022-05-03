use itertools::Itertools;
// use std::time::Instant;
use std::fmt;

use crate::array::Array64;

use wasm_bindgen::prelude::*;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// u64 = 4 bit * 16
// 16x16までしか対応しない
// 実行時間的に10x10が限界か？
// mat は col-major
#[wasm_bindgen]
#[derive(Clone)]
pub struct Latin {
    size: u8,
    mat: [Array64; 16],
}

impl Latin {
    pub fn new(size: u8) -> Self {
        assert!(size < 16);

        Latin {
            size: size,
            mat: [Array64::new(); 16],
        }
    }
    pub fn from_vec(vec: Vec<Vec<u8>>) -> Self {
        let size = vec.len();
        let mut latin = Latin::new(size as u8);
        for i in 0..size {
            assert_eq!(size, vec[i].len());
            for j in 0..size {
                latin.set(i as u8, j as u8, vec[i][j]);
            }
        }

        latin
    }
    pub fn valid(&self) -> bool {
        for i in 0..self.size() {
            // check row: i
            let mut hist = vec![0u8; self.size() as usize];
            for j in 0..self.size() {
                hist[self.get(i, j) as usize] += 1;
            }
            for h in hist.iter() {
                if *h != 1 {
                    return false;
                }
            }
        }
        for j in 0..self.size() {
            // check col: j
            let mut hist = vec![0u8; self.size() as usize];
            for i in 0..self.size() {
                hist[self.get(i, j) as usize] += 1;
            }
            for h in hist.iter() {
                if *h != 1 {
                    return false;
                }
            }
        }

        true
    }
}

pub fn orthogonal(lhs: &Latin, rhs: &Latin) -> bool {
    if lhs.size() != rhs.size() || !lhs.valid() || !rhs.valid() {
        return false;
    }

    let size = lhs.size() as usize;
    let mut hist = vec![vec![0u8; size]; size];
    for i in 0..size {
        for j in 0..size {
            let x = lhs.get(i as u8, j as u8);
            let y = rhs.get(i as u8, j as u8);
            hist[x as usize][y as usize] += 1;
        }
    }
    for i in 0..size {
        for j in 0..size {
            if hist[i][j] != 1 {
                return false;
            }
        }
    }

    true
}

#[wasm_bindgen]
impl Latin {
    pub fn mynew() -> Self {
        Latin::from_vec(vec![
            vec![4, 5, 6, 0, 1, 2, 3],
            vec![5, 6, 0, 1, 2, 3, 4],
            vec![6, 0, 1, 2, 3, 4, 5],
            vec![0, 1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 6, 0],
            vec![2, 3, 4, 5, 6, 0, 1],
            vec![3, 4, 5, 6, 0, 1, 2],
        ])
    }
    pub fn transversal(&self) -> TransversalList {
        TransversalList {
            transversal_list: get_transversals(self).into_iter().flatten().collect(),
        }
    }
    pub fn orthogonal(&self) -> Latin {
        search_orthogonal_latin(self, false).pop().unwrap()
    }
    pub fn size(&self) -> u8 {
        self.size
    }
    pub fn set(&mut self, i: u8, j: u8, v: u8) {
        debug_assert!(i < self.size);
        debug_assert!(j < self.size);
        debug_assert!(v < self.size);

        self.mat[j as usize].set(i as usize, v)
    }
    pub fn get(&self, i: u8, j: u8) -> u8 {
        debug_assert!(i < self.size);
        debug_assert!(j < self.size);

        self.mat[j as usize].get(i as usize)
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Latin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.size() {
            for j in 0..self.size() {
                write!(f, "{}", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Transversal(u64);

impl Transversal {
    pub fn new() -> Self {
        Transversal(0)
    }
    pub fn from_vec(vec: Vec<u8>) -> Self {
        let mut t = 0u64;
        for x in vec.into_iter().rev() {
            t <<= 1 << 2;
            t |= x as u64;
        }
        Transversal(t)
    }
}

#[wasm_bindgen]
impl Transversal {
    pub fn set(&mut self, j: u8, v: u8) {
        // set 0
        let mask = 0xfu64 << (j << 2);
        let mask = !mask;
        self.0 &= mask;
        // set v
        let v = (v as u64) << (j << 2);
        self.0 |= v;
    }
    pub fn get(&self, j: u8) -> u8 {
        let mask = 0xfu64;
        ((self.0 >> (j << 2)) & mask) as u8
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TransversalList {
    transversal_list: Vec<Transversal>,
}

#[wasm_bindgen]
impl TransversalList {
    pub fn size(&self) -> usize {
        self.transversal_list.len()
    }
    pub fn get(&self, idx: usize) -> Transversal {
        self.transversal_list[idx]
    }
}

pub fn get_transversals(latin: &Latin) -> Vec<Vec<Transversal>> {
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

pub fn pretty_print_transversal(latin: &Latin, transversal: Transversal) {
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

pub fn pretty_print_latin(latin: &Latin) {
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
    board: &mut [[u8; 16]; 16],
    size: u8,
    search: u8,
    answers: &mut Vec<Vec<Transversal>>,
) {
    debug_assert_eq!(state.len(), search as usize);

    let transversals = &transversal_map[search as usize];

    for t in transversals.iter() {
        // state に t を追加して矛盾が生じないかチェックする
        let mut ok = true;
        for j in 0..size {
            if board[j as usize][t.get(j) as usize] == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            state.push(*t);
            for j in 0..size {
                board[j as usize][t.get(j) as usize] = 1;
            }

            if search + 1 == size {
                answers.push(state.clone());
            } else {
                dfs(transversal_map, state, board, size, search + 1, answers);
            }

            state.pop();
            for j in 0..size {
                board[j as usize][t.get(j) as usize] = 0;
            }
        }
    }
}

pub fn search_orthogonal_latin(latin: &Latin, verbose: bool) -> Vec<Latin> {
    // let start = Instant::now();
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
    let mut board = [[0u8; 16]; 16];
    let mut answers = vec![];
    state.reserve(100);
    answers.reserve(100);
    dfs(
        &transversal_map,
        &mut state,
        &mut board,
        latin.size(),
        0,
        &mut answers,
    );

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

    // let end = start.elapsed();
    // if verbose {
    //     println!(
    //         "Search time of orthogonal latin squares: \n\t{}.{:03} [sec]",
    //         end.as_secs(),
    //         end.subsec_nanos() / 1_000_000
    //     );
    // }

    ret
}

pub fn solve(matrix: Vec<Vec<u8>>) {
    // Construct latin square
    let latin = Latin::from_vec(matrix);
    if !latin.valid() {
        println!("Invalid input");
        panic!();
    }
    println!("Input latin square: ");
    pretty_print_latin(&latin);

    // Search orthogonal latin squares
    let orthogonals = search_orthogonal_latin(&latin, true);
    for ortho in orthogonals.iter() {
        assert!(orthogonal(&latin, ortho));
    }

    // Show results
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
