use oxidizer::file_io;
use oxidizer::kanji_provider;
use oxidizer::types::FilePaths;
use oxidizer::types::Settings;
use std::collections::HashSet;
use std::error::Error;
use std::io;

// specialChars { '、', '/', '~' }

/// Vytváří soubor s výsledky voláním metod pro zpracování
pub fn transform(settings: &Settings) -> Result<Vec<String>, Box<Error>> {
    let file_paths = file_io::create_file_paths(&settings);
    let lines = get_lines(&file_paths)?;
    find_invalid_lines(&lines)?;
    let converted_lines = convert_word_list(&lines);

    file_io::write_all_lines(&file_paths.output_file_path.as_str(), &converted_lines)?;

    Ok(converted_lines)
}

/// Čte soubor se slovíčky nebo Kanji v závislosti na nastavení
pub fn get_lines<'a>(file_paths: &'a FilePaths) -> io::Result<Vec<String>> {
    println!("Soubor: {}", &file_paths.word_file_path);
    let lines = file_io::read_all_lines(&file_paths.word_file_path)?;

    Ok(lines.into_iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

/// Vybírá řádky, které nejsou prázdné. V případě nastavení use_subset,
/// používá starting_line a ending line pro vybrání části řádků.
/// Výsledek pod výběru bude obsahovat staring_line, ale nebude obsahovat ending_line.
pub fn filter_lines<'a>(lines: Vec<String>) -> Vec<String> {
    lines.into_iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn find_invalid_lines(lines: &[String]) -> Result<(), String> {
    use std::fmt::Write;

    let invalid_lines: Vec<String> = lines.iter()
        .filter(|line| !line.contains('\t'))
        .map(|line| line.to_owned())
        .collect();

    if !invalid_lines.is_empty() {
        let mut error_message = String::new();
        writeln!(&mut error_message, "Následující řádky nemohou být rozděleny: \n").unwrap();

        for invalid_line in &invalid_lines {
            writeln!(&mut error_message, "{}", invalid_line).unwrap();
        }
        Err(error_message)
    }
    else { Ok(()) }
}

fn convert_word_list(lines: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for line in lines.iter() {
        let (japanese, meaning) = split(line);
        let use_kanji_marker = contains_kanji(japanese);
        let use_tilde_marker = japanese.contains("~");

        let marker =
        match (use_kanji_marker, use_tilde_marker) {
            (true, true) => " (~/K)",
            (false, true) => " (~)",
            (true, false) => " (K)",
            _ => ""
        };

        result.push(format!("{}|{}{}", japanese, meaning, marker));
    }

    result
}

fn contains_kanji(string: &str) -> bool {
    let chars: HashSet<char> = string.chars().collect();
    let kanji = kanji_provider::get_kanji_chars();

    !kanji.is_disjoint(&chars)
}

fn split<'a>(s: &'a String) -> (&'a str, &'a str) {
    let index = s.find("\t").unwrap();
    let (l_str, r_str) = s.split_at(index);

    (l_str.trim(), r_str.trim())
}