use std::marker::{Send, Sync};
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
        let chunk_size = (len + NUM_CPU - 1) / NUM_CPU;
        println!("chunk size={}", chunk_size);
        let chunks = v.chunks(chunk_size);
        let chunks_len = chunks.len();
        println!("chunks len={}", chunks_len);
        let mut threads = Vec::new();
        for chunk in chunks {
            let ch = chunk.to_owned();
            threads.push(std::thread::spawn(move || vec_f(ch, f)));
        }
        threads
            .into_iter()
            .map(|t| t.join().unwrap())
            .collect::<Vec<Vec<U>>>()
            .concat()
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
