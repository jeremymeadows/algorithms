#![feature(once_cell)]
#![feature(bench_black_box)]
#![feature(optimize_attribute)]

use std::lazy::SyncLazy;
use std::hint;

//#[optimize(none)]
static VAL: SyncLazy<String> = SyncLazy::new(|| { hint::black_box(String::from_utf8(
        [ 140, 155, 158, 142, 158, 142, 166, 156, 150, 130, 148, 171, 128, 134, 132, 150, 137, ]
        .iter()
        .enumerate()
        .map(|(i, e)| i as u8 ^ !e as u8)
        .collect::<Vec<u8>>(),
        )).unwrap()});

fn main() {
    println!("{}", *VAL);
}
