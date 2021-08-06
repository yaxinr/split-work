use std::sync::mpsc;
fn main() {
    // println!("Number of logical cores is {}", num_cpus::get());
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let f = |x: i32| x * 2;
    let r = split_work(v, f);
    for i in r {
        println!("{}", i)
    }
}

fn split_work<T: Copy + std::marker::Send + 'static, F>(v: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(T) -> T + Send + Copy + 'static + std::marker::Sync,
{
    let num_cpu = 2;
    let mut r = Vec::new();
    let (tx, rx) = mpsc::channel();
    let chunk_size = v.len() / num_cpu;
    let chunks = v.chunks(chunk_size);
    for chunk in chunks {
        let ch = chunk.to_vec();
        let tx1 = tx.clone();
        std::thread::spawn(move || {
            for i in ch.clone().into_iter() {
                let j = f(i.clone());
                tx1.send(j).unwrap();
            }
        });
    }

    let mut i = v.len();
    loop {
        let received = rx.recv().unwrap();
        r.push(received);
        i -= 1;
        if i == 0 {
            break;
        };
    }
    r
}
