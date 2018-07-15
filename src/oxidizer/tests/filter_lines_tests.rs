use oxidizer::core;
use oxidizer::types::Settings;

#[test]
fn gets_entire_file() {
    let data = default_vec_values();
    let settings = default_test_settings();

    let result = core::filter_lines(&settings, data);

    assert_eq!(5, result.len());
}

#[test]
fn skips_empty_line() {
    let data = default_vec_values_some_empty();
    let settings = default_test_settings();

    let result = oxidizer::filter_lines(&settings, data);

    assert_eq!(4, result.len());
}

#[test]
fn gets_subset() {
    let data = default_vec_values();
    let mut settings = default_test_settings();
    settings.use_subset = true;

    let result = oxidizer::filter_lines(&settings, data);

    assert_eq!(3, result.len());
}

#[test]
fn gets_subset_skips_empty_line() {
    let data = default_vec_values_some_empty();
    let mut settings = default_test_settings();
    settings.use_subset = true;

    let result = oxidizer::filter_lines(&settings, data);

    assert_eq!(2, result.len());
}

fn default_test_settings() -> Settings {
    Settings {
        working_dir: String::new(),
        word_file_name: String::new(),
        kanji_file_name: String::new(),
        output_file_name: String::new(),
        kanji_mode: false,
        use_subset: false,
        starting_line: "1".to_string(),
        ending_line: "4".to_string()
    }
}

fn default_vec_values_some_empty() -> Vec<String> {
    vec![ "0".to_string(),
          "1".to_string(),
          "".to_string(),
          "3".to_string(),
          "4".to_string() ]
}

fn default_vec_values() -> Vec<String> {
    vec![ "0".to_string(),
          "1".to_string(),
          "2".to_string(),
          "3".to_string(),
          "4".to_string() ]
}