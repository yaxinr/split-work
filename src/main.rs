use std::{
    marker::{Send, Sync},
    sync::mpsc,
};
const THRESHOLD: usize = 2;
fn main() {
    // println!("Number of logical cores is {}", num_cpus::get());
    let v = vec!["1", "22", "333", "4", "5", "6", ""];
    let f = |x: &str| x.len();
    let r = split_work(v, f);
    for i in r {
        println!("{}", i)
    }
    let v = vec![1, 2, 3, 4, 5, 6];
    let f = |x: i32| x * 2;
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
    const NUM_CPU: usize = 2;
    let len = v.len();
    println!("len={}", len);
    if len > THRESHOLD {
        let (tx, rx) = mpsc::channel();
        let chunk_size = (len + NUM_CPU - 1) / NUM_CPU;
        println!("chunk size={}", chunk_size);
        let chunks = v.chunks(chunk_size);
        let chunks_len = chunks.len();
        println!("chunks len={}", chunks_len);
        for (i, chunk) in chunks.enumerate() {
            let ch = chunk.to_vec();
            let tx1 = tx.clone();
            std::thread::spawn(move || tx1.send((i, vec_f(ch, f))).expect("send error"));
        }
        drop(tx);
        let mut r: Vec<U> = Vec::with_capacity(len);
        unsafe {
            r.set_len(len);
        }
        for received in rx.iter() {
            for (j, u) in received.1.into_iter().enumerate() {
                r[received.0 * chunk_size + j] = u;
            }
        }
        r
    } else {
        vec_f(v, f)
    }
}

fn vec_f<T, U, F>(ch: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U + Copy + Send + 'static + Sync,
    T: Copy + Send + 'static,
    U: Copy + Send + 'static,
{
    ch.into_iter().map(|t| f(t)).collect()
}
