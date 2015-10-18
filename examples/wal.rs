extern crate env_logger;
extern crate eventual;
extern crate wal;

use std::env;

use eventual::{Async, Future, Join};

use wal::Wal;

fn main() {
    let _ = env_logger::init();
    let path = env::args().skip(1).next().unwrap_or(".".to_owned());
    println!("path: {}", path);
    let mut wal = Wal::open(&path).unwrap();

    let entry: &[u8] = &[42u8; 4096];
    let mut completions = Vec::with_capacity(10000);

    for _ in 1..100 {
        completions.push(wal.append(&entry));
    }

    let (c, f) = Future::pair();
    completions.join(c);
    f.await().unwrap();
}
