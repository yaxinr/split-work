use std::sync::mpsc;
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

fn split_work<
    T: Copy + std::marker::Send + 'static,
    U: Copy + std::marker::Send + 'static + Default,
    F,
>(
    v: Vec<T>,
    f: F,
) -> Vec<U>
where
    F: Fn(T) -> U + Send + Copy + 'static + std::marker::Sync,
{
    // let num_cpu = num_cpus::get();
    const NUM_CPU: usize = 4;
    let len = v.len();
    if len > THRESHOLD {
        let mut r: Vec<U> = vec![U::default(); len];
        let (tx, rx) = mpsc::channel();
        let chunk_size = if len > NUM_CPU { len / NUM_CPU } else { 1 };
        let chunks = v.chunks(chunk_size);
        for (i, chunk) in chunks.enumerate() {
            let ch = chunk.to_vec();
            let tx1 = tx.clone();
            std::thread::spawn(move || {
                for (j, t) in ch.into_iter().enumerate() {
                    let u = f(t);
                    let r = FnResult {
                        i: i * chunk_size + j,
                        v: u,
                    };
                    tx1.send(r).unwrap();
                }
            });
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
