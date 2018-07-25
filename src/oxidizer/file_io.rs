use std::io;
use oxidizer::types::{Settings, FilePaths};
use std::error::Error;

pub fn create_file_paths(settings: &Settings) -> FilePaths {
    let mut word_file_path = settings.working_dir.clone();
    let mut output_file_path = settings.working_dir.clone();

    word_file_path.push_str(&settings.word_file_name);
    output_file_path.push_str(&settings.output_file_name);

    FilePaths {
        word_file_path,
        output_file_path
    }
}

pub fn read_all_lines(file_path: &str) -> io::Result<Vec<String>> {
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);

    let lines: Vec<String> = buf_reader.lines()
        .map(|l| l.unwrap())
        .collect();

    Ok(lines)
}

pub fn write_all_lines(file_path: &str, lines: &[String]) -> Result<(), Box<Error>> {
    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::fs::File;

    let file = File::create(file_path)?;
    let mut buf_writer = BufWriter::new(file);

    for line in lines.iter() {
        let l = format!("{}\r\n", line);
        buf_writer.write(l.as_ref())?;
    }

    Ok(())
}