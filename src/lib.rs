#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Index;
#[macro_export]
macro_rules! pink_elephant {
    () => {
        let a = 4;
    };
}
#[macro_export]
macro_rules! blue_wail {
    ($s:literal) => {
        println!("Hello ther blue wail literal {}", $s);
    };
}
#[macro_export]
macro_rules! sky {
    ($var: ident) => {
        println!("Hello this is literal {}", $var);
    };
}

#[macro_export]
macro_rules! amyvec {
    () => {
        Vec::new()
    };

    ($($el:expr),+ $(,)?) => {{
        let mut x = Vec::new();
        $(x.push($el);)+
        x
    }};
    ($e:expr; $count:expr) => {{
        let count = $count;
        let x = $e;
        let mut vec = Vec::with_capacity(count);
        //no re-allocation, no boundary checks
        vec.extend(std::iter::repeat($e).take(count));
        // bad practice
        // re-allocates every time when exeeds capacity (default +16? or something like that)
        // for _ in 0..count {
        //     vec.push(x.clone());
        // }
        vec
    }}
}

/// doc text about this function
pub fn macro_wrapper() {
    let fib = {
        struct IndexOffset {
            slice: [u64; 2],
            offset: usize,
        }

        impl Index<usize> for IndexOffset {
            type Output = u64;

            fn index(&self, index: usize) -> &u64 {
                use std::num::Wrapping;

                let index_w = Wrapping(index);
                let offset_w = Wrapping(self.offset);
                let window_w = Wrapping(2);
                let real_index = index_w - offset_w + window_w;
                &self.slice[real_index.0]
            }
        }
        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }

        impl Iterator for Recurrence {
            type Item = u64;
            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_value = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_value)
                } else {
                    let next_value = {
                        let n = self.pos;
                        let a = IndexOffset {
                            slice: self.mem,
                            offset: n,
                        };
                        a[n - 2] + a[n - 1]
                    };
                    {
                        use std::mem::swap;
                        let mut swap_buff = next_value;
                        for i in [1, 0] {
                            swap(&mut swap_buff, &mut self.mem[i]);
                        }
                        self.pos += 1;
                        Some(next_value)
                    }
                }
            }
        }
        Recurrence {
            mem: [0, 1],
            pos: 0,
        }
    };
    for e in fib.take(10) {
        println!("final: {}", e);
    }
}

#[test]
fn test1() {
    pink_elephant!();
}
#[test]
fn sky_ident() {
    let u = 44;
    sky!(u);
}
#[test]
fn blue_wail_literal() {
    blue_wail!("FRED");
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = amyvec!();
    assert!(x.is_empty());
}

#[test]
fn single_element() {
    let x: Vec<u32> = amyvec![44];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 44);
}

#[test]
fn double_element() {
    let y: Vec<u32> = amyvec!(22, 33);
    assert!(!y.is_empty());
    assert_eq!(y.len(), 2);
    assert_eq!(y[0], 22);
    assert_eq!(y[1], 33);
}

#[test]
fn trailing_coma_present() {
    let y: Vec<u32> = amyvec!(22, 33,);
    assert!(!y.is_empty());
    assert_eq!(y.len(), 2);
    assert_eq!(y[0], 22);
    assert_eq!(y[1], 33);
}
#[test]
fn vec_with_capacity() {
    let y: Vec<u32> = amyvec!(22; 2);
    assert!(!y.is_empty());
    assert_eq!(y.len(), 2);
    assert_eq!(y[0], 22);
    assert_eq!(y[1], 22);
}
