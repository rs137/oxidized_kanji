/// Nastavení pro proces zpracování souboru se slovíčky
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    /// Adresář obsahující soubory se slovíčky
    pub working_dir: String,

    /// Název souboru se slovíčky
    pub word_file_name: String,

    /// Název souboru s výsledky zpracování
    pub output_file_name: String
}

/// Sdružuje všechny cesty k souborům
pub struct FilePaths {
    /// Celá cesta k souboru se slovíčky
    pub word_file_path: String,

    /// Celá cesta k souboru s výsledky
    pub output_file_path: String
}