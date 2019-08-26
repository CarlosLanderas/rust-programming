use features::executors::{MyExecutor, Executor};

fn main() {
    #[cfg(feature = "string-exec")]
    femme::start(log::LevelFilter::Debug).unwrap();

    let e = MyExecutor{};
    for e in e.run() {
        println!("{}", e);
    }
}
