use anvil_core::password::Generator;

use crate::dto::{password_to_password_dto, PasswordResultDto};

#[tauri::command]
pub fn generate_password(
    with_number: Option<bool>,
    with_uppercase: Option<bool>,
    with_lowercase: Option<bool>,
    with_symbols: Option<bool>,
    with_extended_ascii: Option<bool>,
) -> PasswordResultDto {
    let mut generator = Generator::new();

    if let Some(v) = with_number {
        generator.with_numbers(v);
    }
    if let Some(v) = with_uppercase {
        generator.with_uppercase(v);
    }
    if let Some(v) = with_lowercase {
        generator.with_lowercase(v);
    }
    if let Some(v) = with_symbols {
        generator.with_symbols(v);
    }
    if let Some(v) = with_extended_ascii {
        generator.with_extended_ascii(v);
    }

    password_to_password_dto(generator.generate())
}
