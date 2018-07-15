use std::io;
use oxidizer::types::{Settings, FilePaths};

pub fn create_file_paths(settings: &Settings) -> FilePaths {
    let mut kanji_file_path = settings.working_dir.clone();
    let mut word_file_path = settings.working_dir.clone();
    let mut output_file_path = settings.working_dir.clone();

    kanji_file_path.push_str(&settings.kanji_file_name);
    word_file_path.push_str(&settings.word_file_name);
    output_file_path.push_str(&settings.output_file_name);

    FilePaths {
        kanji_file_path,
        word_file_path,
        _output_file_path: output_file_path
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