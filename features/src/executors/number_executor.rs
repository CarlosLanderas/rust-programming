
use super::{Executor, MyExecutor};

impl Executor for MyExecutor {
    type Item = usize;

    fn run(&self) -> Vec<Self::Item> {
        vec![2, 3, 5]
    }
}
