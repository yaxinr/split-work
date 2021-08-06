fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
    let mut v = Vec::new();
    v.push(1);
    let f = |x: i32| x;
    split_work(v, f2);
}

fn split_work<T>(v: Vec<T>, &f: dyn Fn(T) -> T) -> Vec<T> {
    let num_cpu = num_cpus::get();

    f(v[0]);
    // for i in 0..5 {
    //     std::thread::spawn(move || {
    //         let args = Docopt::new(USAGE)
    //             .and_then(|dopt| dopt.parse())
    //             .unwrap_or_else(|e| e.exit());
    //         let config = mssql_config(args.to_owned());
    //         // let tcp: TcpStream;
    //         // let mut client: Client<TcpStream>;
    //         // println!("  instance: {}", instance);
    //         // if instance == "" {
    //         // config.port(1433);
    //     });
    // }
    v
}

fn f2(x: i32) {
    x
}
