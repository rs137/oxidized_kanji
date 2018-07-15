use oxidizer::types::Settings;
use oxidizer::file_io;
use std::error::Error;
use oxidizer::types::FilePaths;
use std::io;

// specialChars { '、', '/', '~' }

/// Vytváří soubor s výsledky voláním metod pro zpracování
pub fn transform(settings: Settings) -> Result<Vec<String>, Box<Error>> {
    let file_paths = file_io::create_file_paths(&settings);

    let mut lines = get_lines(&settings, &file_paths)?;
    lines = filter_lines(&settings, lines);


    Ok(lines)
}

/// Čte soubor se slovíčky nebo Kanji v závislosti na nastavení
pub fn get_lines<'a>(settings: &'a Settings, file_paths: &'a FilePaths) -> io::Result<Vec<String>> {
    let lines;
    if settings.kanji_mode {
        lines = file_io::read_all_lines(&file_paths.kanji_file_path)?;
    } else {
        lines = file_io::read_all_lines(&file_paths.word_file_path)?;
    }

    Ok(lines)
}

/// Vybírá řádky, které nejsou prázdné. V případě nastavení use_subset,
/// používá starting_line a ending line pro vybrání části řádků.
/// Výsledek pod výběru bude obsahovat staring_line, ale nebude obsahovat ending_line.
pub fn filter_lines<'a>(settings: &'a Settings, lines: Vec<String>) -> Vec<String> {
    let use_subset = settings.use_subset;

    lines.into_iter()
        .map(|line| line.trim().to_string())
        .skip_while(|line| {
            if use_subset {
                line != &settings.starting_line
            } else {
                false
            }
        })
        .filter(|line| !line.is_empty())
        .take_while(|line| {
            if use_subset {
                line != &settings.ending_line
            } else {
                true
            }
        })
        .collect()
}

