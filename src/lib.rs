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

pub fn macro_wrapper() {
    // let u = 44;
    // pink_elephant!();
    // blue_wail!("FRED");
    // sky!(u);

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
