use std::{
    marker::{Send, Sync},
    sync::mpsc,
};
const THRESHOLD: usize = 8;
fn main() {
    // println!("Number of logical cores is {}", num_cpus::get());
    // let v = vec![1, 2, 3, 4, 5, 6];
    let v = vec!["1", "22", "333", "4", "5", "6"];
    // let f = |x: i32| x * 2;
    let f = |x: &str| x.len();
    let r = split_work(v, f);
    for i in r {
        println!("{}", i)
    }
}

fn split_work<T, U, F>(v: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U + Copy + Send + 'static + Sync,
    T: Copy + Send + 'static,
    U: Copy + Send + 'static,
{
    // let num_cpu = num_cpus::get();
    const NUM_CPU: usize = 4;
    let len = v.len();
    if len > THRESHOLD {
        let (tx, rx) = mpsc::channel();
        let chunk_size = if len > NUM_CPU { len / NUM_CPU } else { 1 };
        let chunks = v.chunks(chunk_size);
        for (i, chunk) in chunks.enumerate() {
            let ch = chunk.to_vec();
            let tx1 = tx.clone();
            std::thread::spawn(move || {
                for (j, t) in ch.iter().enumerate() {
                    let u = f(*t);
                    let r = FnResult {
                        i: i * chunk_size + j,
                        v: u,
                    };
                    tx1.send(r).unwrap();
                }
            });
        }

        let mut r: Vec<U> = Vec::with_capacity(len);
        unsafe {
            r.set_len(len);
        }
        let mut i = len;
        loop {
            let received = rx.recv().unwrap();
            r[received.i] = received.v;
            i -= 1;
            if i == 0 {
                break;
            };
        }
        r
    } else {
        v.into_iter().map(|t| f(t)).collect()
    }
}

struct FnResult<T> {
    i: usize,
    v: T,
}
