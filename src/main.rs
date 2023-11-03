use std::{fs, io};

fn main() -> io::Result<()> {
    let entries = fs::read_dir("./")?;

    for entry in entries {
        let entry = entry?;
        print!("{}", entry.path().display())
    }

    Ok(())
}
