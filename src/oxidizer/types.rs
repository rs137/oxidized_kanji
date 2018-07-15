/// Nastavení pro proces zpracování souboru se slovíčky
#[derive(Debug)]
pub struct Settings {
    /// Adresář obsahující soubory se slovíčky
    pub working_dir: String,

    /// Název souboru s kanji
    pub kanji_file_name: String,

    /// Název souboru se slovíčky
    pub word_file_name: String,

    /// Název souboru s výsledky zpracování
    pub output_file_name: String,

    /// Přepínáč pro zpracování souborů s kanji nebo se slovíčky
    pub kanji_mode: bool,

    /// Přepínač pro částečné zpracování souboru
    pub use_subset: bool,

    /// Udává řádek, od kterého se má soubor začít zpracovávat
    ///
    /// Aktivní v případě nastavení use_subset na true
    ///
    /// Musí obsahovat celý řádek
    pub starting_line: String,

    /// Udává řádek, po který se má soubor zpracovat
    ///
    /// Aktivní v případě nastavení use_subset na true
    ///
    /// Musí obsahovat celý řádek
    pub ending_line: String
}

/// Sdružuje všechny cesty k souborům
pub struct FilePaths {
    /// Celá cesta ke kanji souboru
    pub kanji_file_path: String,

    /// Celá cesta k souboru se slovíčky
    pub word_file_path: String,

    /// Celá cesta k souboru s výsledky
    pub _output_file_path: String
}