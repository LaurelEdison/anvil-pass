use anvil_core::{password::Generator, vault::models::PasswordResult};

#[tauri::command]
pub fn generate_password(
    with_number: Option<bool>,
    with_uppercase: Option<bool>,
    with_lowercase: Option<bool>,
    with_symbols: Option<bool>,
    with_extended_ascii: Option<bool>,
) -> PasswordResult {
    let mut generator = Generator::new();
    if with_number.is_some() {
        generator.with_numbers(with_number.unwrap_or(true));
    }
    if with_uppercase.is_some() {
        generator.with_uppercase(with_uppercase.unwrap_or(true));
    }
    if with_lowercase.is_some() {
        generator.with_lowercase(with_lowercase.unwrap_or(true));
    }
    if with_symbols.is_some() {
        generator.with_symbols(with_symbols.unwrap_or(true));
    }
    if with_extended_ascii.is_some() {
        generator.with_extended_ascii(with_extended_ascii.unwrap_or(false));
    }

    generator.generate()
}
