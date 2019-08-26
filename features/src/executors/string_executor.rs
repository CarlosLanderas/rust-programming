use crate::executors::{MyExecutor, Executor};

impl Executor for MyExecutor {
    type Item = String;

    fn run(&self) -> Vec<Self::Item> {
        log::debug!("Creating vec of strings");
        vec!["this".to_string(), "is".to_string(), "cinnamon".to_string()]
    }
}
