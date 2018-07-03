#![feature(try_trait)]

use std::fs;
use std::io::{Read, BufWriter, Write};

#[derive(Debug)]
enum ConvertError {
    Arg(std::option::NoneError),
    IO(std::io::Error),
    UTF8(std::str::Utf8Error),
}

impl std::convert::From<std::option::NoneError> for ConvertError {
    fn from(error : std::option::NoneError) -> ConvertError {
        ConvertError::Arg(error)
    }
}

impl std::convert::From<std::io::Error> for ConvertError {
    fn from(error : std::io::Error) -> ConvertError {
        ConvertError::IO(error)
    }
}

impl std::convert::From<std::str::Utf8Error> for ConvertError {
    fn from(error : std::str::Utf8Error) -> ConvertError {
        ConvertError::UTF8(error)
    }
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ConvertError::*;
        
        match self {
            Arg(e) => write!(f, "{:?}", e),
            IO(e) => write!(f, "{:?}", e),
            UTF8(e) => write!(f, "{:?}", e),
        }
    }
}

fn format (filename : &String, source : String) -> String {
    let filename = filename.replace(".", "-");
    format!("let {} = '<+code-box(```\n{}\n```);>\n\n", filename, source)
}


fn main() -> Result<(), ConvertError> {
    let dir = "source";
    println!("Get directory name");

    let paths = fs::read_dir(dir)?;
    println!("Read directory");

    let file = fs::File::create("saty/source.satyh")?;
    println!("Create file");

    let mut f = BufWriter::new(file);
    f.write(b"@import: local\n\n")?;
    
    for path in paths {
        let path = path.unwrap().path();

        if !path.display().to_string().contains(".c") {
            continue;
        }

        let mut sf = fs::File::open(path.clone())?;
        let mut buf = vec![];
        sf.read_to_end(&mut buf)?;
        let source = std::str::from_utf8(&buf)?;
        println!("Read file: {}", path.display());

        let filename = &path.display().to_string().replace("source/", "").replace("/", "-");
        f.write(format(filename, source.to_string()).as_bytes())?;
        println!("Write file: {}", path.display());
    }

    println!("Complete");
    Ok(())
}