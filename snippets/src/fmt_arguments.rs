#![allow(unused_macros)]

#[allow(dead_code)]
fn format_message(entry: std::fmt::Arguments) -> String {
    use std::fmt::Write;

    let mut message = String::new();
    message.write_fmt(entry);
    message
}

macro_rules! log {
    ($format:tt, $($arg:expr), *) => {
        format_message(format_args!($format, $($arg), *));
    };
}

#[test]
fn fmt_arguments_test() {
    let msg = log!("The message is {:?}", "secret message");
    assert_eq!(msg, "The message is \"secret message\"");
    println!("{}", msg);
}
