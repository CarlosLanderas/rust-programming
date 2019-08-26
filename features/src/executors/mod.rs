use std::future::Future;
pub trait Executor {
    type Item;

    fn run(&self) -> Vec<Self::Item>;
}

pub struct MyExecutor;

#[cfg(feature = "string-exec")]
pub mod string_executor;

#[cfg(feature = "num-exec")]
pub mod number_executor;