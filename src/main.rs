/// Modul obsahuje metody pro zpracování jednotlivých řádků se slovíčky
pub mod oxidizer;

use oxidizer::types::Settings;
use oxidizer::core;

static APP_NAME: &str = "Oxidized Kanji";
static VERSION: &str = "0.1.0";

fn main() {
    print_header();

    let settings = create_default_settings();
    let result = core::transform(settings);

    match result {
        Ok(lines) => {
            for line in lines.iter() {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("{}", e.description())
    }
}

fn create_default_settings() -> Settings {
    Settings {
        working_dir: r#"D:\USB Drive\Backup\Japonština\Slovíčka\"#.to_string(),
        word_file_name: "43-51.txt".to_string(),
        kanji_file_name: "Kanji.txt".to_string(),
        output_file_name: "output.txt".to_string(),
        kanji_mode: false,
        use_subset: false,
        starting_line: "日	Slunce".to_string(),
        ending_line: "新	new".to_string()
    }
}

fn print_header() {
    println!("{} [{}]\n", APP_NAME, VERSION);
}