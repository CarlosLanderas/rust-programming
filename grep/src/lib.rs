use std::io::{self, BufRead};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn grep_fn<R: BufRead>(target: &str, reader: R) -> io::Result<()> {
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }

    Ok(())
}