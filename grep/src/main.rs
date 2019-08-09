use grep::grep_fn;
use std::io::{self, BufReader, Write};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;


fn main() {
    let result = grep_main();
    if let Err(err) = result {
        let _ = writeln!(io::stderr(), "{}", err);
    }
}


fn grep_main() -> Result<(), Box<Error>> {

    let mut args = std::env::args().skip(1);

    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE")?
    };

    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        let stdin = io::stdin();
        grep_fn(&target, stdin.lock())?;
    }
    else {
        for file in files {
            let f = File::open(file)?;
            grep_fn(&target, BufReader::new(f))?;
        }
    }

    Ok(())
}
