#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

/// Modul obsahuje metody pro zpracování jednotlivých řádků se slovíčky
pub mod oxidizer;

use oxidizer::types::Settings;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use oxidizer::core;

static APP_NAME: &str = "Oxidized Kanji";
static VERSION: &str = "0.1.0";

fn main() {
    print_header();

    let settings = load_settings().unwrap_or_else(|_| {
        println!("Konfiguraci se nepodařilo načíst\nNačítám výchozí konfiguraci.");
        create_default_settings()
    });

    let result = core::transform(&settings);

    match result {
        Ok(lines) => {
            for line in &lines {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("{}", e.description())
    }
}

fn create_default_settings() -> Settings {
    Settings {
        working_dir: r#"D:\USB Drive\Backup\Japonština\Slovíčka\"#.to_string(),
        word_file_name: "input.txt".to_string(),
        output_file_name: "output.txt".to_string()
    }
}

fn load_settings() -> Result<Settings, Box<Error>> {
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let settings: Settings = serde_json::from_str(&contents)?;

    Ok(settings)
}

fn print_header() {
    println!("{} [{}]\n", APP_NAME, VERSION);
}