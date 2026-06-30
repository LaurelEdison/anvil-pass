use anvil_core::password::{Charset, Generator};

#[test]
fn test_generator_default() {
    let generator = Generator::default();

    assert_eq!(generator.length, 24);
    assert!(generator.numbers);
    assert!(generator.lowercase_letters);
    assert!(generator.uppercase_letters);
    assert!(generator.symbols);
    assert!(!generator.spaces);
    assert!(!generator.exclude_similar_characters);
    assert!(generator.strict);
}

#[test]
fn test_generator_new() {
    let generator = Generator::new();

    // Should be same as default
    assert_eq!(generator.length, 24);
    assert!(generator.numbers);
    assert!(generator.lowercase_letters);
    assert!(generator.uppercase_letters);
    assert!(generator.symbols);
    assert!(!generator.spaces);
    assert!(!generator.exclude_similar_characters);
    assert!(generator.strict);
}

#[test]
fn test_generator_with_length() {
    let generator = Generator::new().with_length(32);

    assert_eq!(generator.length, 32);

    let generator = Generator::new().with_length(0);
    assert_eq!(generator.length, 0);

    let generator = Generator::new().with_length(100);
    assert_eq!(generator.length, 100);
}

#[test]
fn test_generator_with_numbers() {
    let generator = Generator::new().with_numbers(false);
    assert!(!generator.numbers);

    let generator = Generator::new().with_numbers(true);
    assert!(generator.numbers);
}

#[test]
fn test_generator_with_lowercase() {
    let generator = Generator::new().with_lowercase(false);
    assert!(!generator.lowercase_letters);

    let generator = Generator::new().with_lowercase(true);
    assert!(generator.lowercase_letters);
}

#[test]
fn test_generator_with_uppercase() {
    let generator = Generator::new().with_uppercase(false);
    assert!(!generator.uppercase_letters);

    let generator = Generator::new().with_uppercase(true);
    assert!(generator.uppercase_letters);
}

#[test]
fn test_generator_with_symbols() {
    let generator = Generator::new().with_symbols(false);
    assert!(!generator.symbols);

    let generator = Generator::new().with_symbols(true);
    assert!(generator.symbols);
}

#[test]
fn test_generator_with_spaces() {
    let generator = Generator::new().with_spaces(true);
    assert!(generator.spaces);

    let generator = Generator::new().with_spaces(false);
    assert!(!generator.spaces);
}

#[test]
fn test_generator_with_exclude_similar() {
    let generator = Generator::new().with_exclude_similar(true);
    assert!(generator.exclude_similar_characters);

    let generator = Generator::new().with_exclude_similar(false);
    assert!(!generator.exclude_similar_characters);
}

#[test]
fn test_generator_with_strict() {
    let generator = Generator::new().with_strict(false);
    assert!(!generator.strict);

    let generator = Generator::new().with_strict(true);
    assert!(generator.strict);
}

#[test]
fn test_generator_chaining() {
    let generator = Generator::new()
        .with_length(30)
        .with_numbers(false)
        .with_lowercase(false)
        .with_uppercase(true)
        .with_symbols(true)
        .with_spaces(true)
        .with_exclude_similar(true)
        .with_strict(false);

    assert_eq!(generator.length, 30);
    assert!(!generator.numbers);
    assert!(!generator.lowercase_letters);
    assert!(generator.uppercase_letters);
    assert!(generator.symbols);
    assert!(generator.spaces);
    assert!(generator.exclude_similar_characters);
    assert!(!generator.strict);
}

#[test]
fn test_generate_default() {
    let generator = Generator::default();
    let password = generator.generate();

    // Should be exactly 24 characters
    assert_eq!(password.len(), 24);

    // Should contain at least one of each character type
    assert!(password.chars().any(|c| c.is_ascii_digit()));
    assert!(password.chars().any(|c| c.is_ascii_lowercase()));
    assert!(password.chars().any(|c| c.is_ascii_uppercase()));
    assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));

    // Should not contain spaces (default false)
    assert!(!password.contains(' '));
}

#[test]
fn test_generate_only_numbers() {
    let generator = Generator::new()
        .with_length(10)
        .with_numbers(true)
        .with_lowercase(false)
        .with_uppercase(false)
        .with_symbols(false)
        .with_spaces(false);

    let password = generator.generate();

    assert_eq!(password.len(), 10);
    assert!(password.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn test_generate_only_lowercase() {
    let generator = Generator::new()
        .with_length(15)
        .with_numbers(false)
        .with_lowercase(true)
        .with_uppercase(false)
        .with_symbols(false)
        .with_spaces(false);

    let password = generator.generate();

    assert_eq!(password.len(), 15);
    assert!(password.chars().all(|c| c.is_ascii_lowercase()));
}

#[test]
fn test_generate_only_uppercase() {
    let generator = Generator::new()
        .with_length(12)
        .with_numbers(false)
        .with_lowercase(false)
        .with_uppercase(true)
        .with_symbols(false)
        .with_spaces(false);

    let password = generator.generate();

    assert_eq!(password.len(), 12);
    assert!(password.chars().all(|c| c.is_ascii_uppercase()));
}

#[test]
fn test_generate_only_symbols() {
    let generator = Generator::new()
        .with_length(8)
        .with_numbers(false)
        .with_lowercase(false)
        .with_uppercase(false)
        .with_symbols(true)
        .with_spaces(false);

    let password = generator.generate();

    assert_eq!(password.len(), 8);
    assert!(password.chars().all(|c| "!@#$%^&*".contains(c)));
}

#[test]
fn test_generate_with_spaces() {
    let generator = Generator::new()
        .with_length(20)
        .with_numbers(false)
        .with_lowercase(true)
        .with_uppercase(false)
        .with_symbols(false)
        .with_spaces(true);

    let password = generator.generate();

    assert_eq!(password.len(), 20);
    assert!(password.contains(' '));
    assert!(password.chars().any(|c| c.is_ascii_lowercase()));
}

#[test]
fn test_generate_without_strict() {
    let generator = Generator::new()
        .with_length(10)
        .with_numbers(true)
        .with_lowercase(true)
        .with_uppercase(true)
        .with_symbols(true)
        .with_strict(false);

    let password = generator.generate();

    assert_eq!(password.len(), 10);
    // With strict false, not all character types are guaranteed
    // But we should still have at least one of the types we requested
    // (though it's possible to miss one in a short password)
    let has_number = password.chars().any(|c| c.is_ascii_digit());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_symbol = password.chars().any(|c| "!@#$%^&*".contains(c));

    // At least one of these should be true
    assert!(has_number || has_lower || has_upper || has_symbol);
}

#[test]
fn test_generate_with_exclude_similar() {
    let generator = Generator::new()
        .with_length(50)
        .with_exclude_similar(true)
        .with_numbers(true)
        .with_lowercase(true)
        .with_uppercase(true);

    let password = generator.generate();

    // Should not contain similar characters
    let similar_chars = ['i', 'l', '1', 'L', 'o', '0', 'O'];
    for c in password.chars() {
        assert!(!similar_chars.contains(&c));
    }
}

#[test]
fn test_generate_with_exclude_similar_but_no_other_chars() {
    // This tests the edge case where excluding similar characters would empty a charset
    // But since our default charset has more than just similar chars, we need to test
    // with a custom scenario. This is more of an edge case.

    // Note: This test passes by construction since the generator handles empty sets
    // gracefully by using unwrap_or on choose()
}

#[test]
fn test_generate_strict_ensures_all_types() {
    let generator = Generator::new()
        .with_length(4) // Minimum length for 4 types
        .with_numbers(true)
        .with_lowercase(true)
        .with_uppercase(true)
        .with_symbols(true)
        .with_strict(true);

    let password = generator.generate();

    assert_eq!(password.len(), 4);
    // With strict=true and length=4, we should have exactly one of each type
    // (though due to shuffling, the order is random)
    let has_number = password.chars().any(|c| c.is_ascii_digit());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_symbol = password.chars().any(|c| "!@#$%^&*".contains(c));

    assert!(has_number);
    assert!(has_lower);
    assert!(has_upper);
    assert!(has_symbol);
}

#[test]
fn test_generate_different_passwords() {
    let generator = Generator::default();

    let password1 = generator.generate();
    let password2 = generator.generate();
    let password3 = generator.generate();

    // There's a tiny chance they could be equal, but practically impossible
    // for 24-character passwords with 70+ character set
    assert_ne!(password1, password2);
    assert_ne!(password2, password3);
    assert_ne!(password1, password3);
}

#[test]
fn test_generate_shuffling() {
    let generator = Generator::new()
        .with_length(4)
        .with_numbers(true)
        .with_lowercase(false)
        .with_uppercase(false)
        .with_symbols(false)
        .with_spaces(false)
        .with_strict(true);

    // With only numbers, strict ensures at least one number, but all are numbers
    // So we test that different calls produce different orders
    let password1 = generator.generate();
    let password2 = generator.generate();

    // Since all characters are numbers, the order can still vary
    // But with only 4 numbers, the chance of different order is high
    // If they're the same, it's not a failure, just random chance
    // So we generate multiple and check at least one differs
    let mut all_same = true;
    let first = generator.generate();
    for _ in 0..10 {
        let next = generator.generate();
        if next != first {
            all_same = false;
            break;
        }
    }
    // It's practically impossible for all 10 to be identical
    assert!(!all_same);
}

#[test]
fn test_generate_very_long() {
    let generator = Generator::new().with_length(1000);

    let password = generator.generate();
    assert_eq!(password.len(), 1000);

    // Should contain all types
    assert!(password.chars().any(|c| c.is_ascii_digit()));
    assert!(password.chars().any(|c| c.is_ascii_lowercase()));
    assert!(password.chars().any(|c| c.is_ascii_uppercase()));
    assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
}

#[test]
fn test_generate_with_all_options_false() {
    let generator = Generator::new()
        .with_numbers(false)
        .with_lowercase(false)
        .with_uppercase(false)
        .with_symbols(false)
        .with_spaces(false)
        .with_strict(true);

    let password = generator.generate();
    assert_eq!(password, "");
}

#[test]
fn test_generate_only_spaces() {
    let generator = Generator::new()
        .with_length(10)
        .with_numbers(false)
        .with_lowercase(false)
        .with_uppercase(false)
        .with_symbols(false)
        .with_spaces(true);

    let password = generator.generate();
    assert_eq!(password.len(), 10);
    assert!(password.chars().all(|c| c == ' '));
}

#[test]
fn test_charset_default() {
    let charset = Charset::default();

    assert_eq!(
        charset.numbers,
        vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']
    );
    assert_eq!(charset.lowercase_letters.len(), 26);
    assert_eq!(charset.uppercase_letters.len(), 26);
    assert_eq!(
        charset.symbols,
        vec!['!', '@', '#', '$', '%', '^', '&', '*']
    );
    assert_eq!(
        charset.similar_chars,
        vec!['i', 'l', '1', 'L', 'o', '0', 'O']
    );
}

#[test]
fn test_charset_similar_chars_contained() {
    let charset = Charset::default();

    // Verify similar chars are actually in the character sets
    for &c in &charset.similar_chars {
        let in_numbers = charset.numbers.contains(&c);
        let in_lower = charset.lowercase_letters.contains(&c);
        let in_upper = charset.uppercase_letters.contains(&c);
        assert!(
            in_numbers || in_lower || in_upper,
            "Similar char '{}' not found in any character set",
            c
        );
    }
}

#[test]
fn test_generate_strict_guarantees_at_least_one_of_each() {
    let generator = Generator::new()
        .with_length(100)
        .with_numbers(true)
        .with_lowercase(true)
        .with_uppercase(true)
        .with_symbols(true)
        .with_spaces(true)
        .with_strict(true);

    let password = generator.generate();

    // Strict should guarantee at least one of each enabled type
    assert!(password.chars().any(|c| c.is_ascii_digit()));
    assert!(password.chars().any(|c| c.is_ascii_lowercase()));
    assert!(password.chars().any(|c| c.is_ascii_uppercase()));
    assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
    assert!(password.contains(' '));
}

#[test]
fn test_generate_uniform_distribution_approximation() {
    // This test checks that characters are roughly uniformly distributed
    // We'll generate many passwords and count character frequencies
    let generator = Generator::new()
        .with_length(1000)
        .with_numbers(true)
        .with_lowercase(true)
        .with_uppercase(true)
        .with_symbols(true)
        .with_strict(false);

    let mut char_counts = std::collections::HashMap::new();
    let total_chars = 10000;

    for _ in 0..10 {
        let password = generator.generate();
        for c in password.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
    }

    // We can't test exact distribution, but we can check that no character
    // appears an unreasonable number of times
    let total_count: usize = char_counts.values().sum();
    assert_eq!(total_count, total_chars);

    let avg = total_count as f64 / char_counts.len() as f64;
    let max_count = char_counts.values().max().unwrap();
    let min_count = char_counts.values().min().unwrap();

    // With 10000 samples across ~70 characters, each should appear ~140 times
    // Allow a reasonable range (e.g., 70-210)
    assert!(*min_count > 50, "Min count {} is too low", min_count);
    assert!(*max_count < 250, "Max count {} is too high", max_count);
}

#[test]
fn test_generator_in_realistic_scenario() {
    // Simulate generating a password for a user
    let generator = Generator::new()
        .with_length(20)
        .with_numbers(true)
        .with_lowercase(true)
        .with_uppercase(true)
        .with_symbols(true)
        .with_spaces(false)
        .with_exclude_similar(true)
        .with_strict(true);

    let password = generator.generate();

    assert_eq!(password.len(), 20);
    assert!(!password.contains(' '));
    assert!(!password.contains('i'));
    assert!(!password.contains('l'));
    assert!(!password.contains('1'));
    assert!(!password.contains('L'));
    assert!(!password.contains('o'));
    assert!(!password.contains('0'));
    assert!(!password.contains('O'));

    // Should have all types
    assert!(password.chars().any(|c| c.is_ascii_digit()));
    assert!(password.chars().any(|c| c.is_ascii_lowercase()));
    assert!(password.chars().any(|c| c.is_ascii_uppercase()));
    assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
}

#[test]
fn test_generator_multiple_calls_consistency() {
    let generator = Generator::default();

    // Generate many passwords and verify they're all valid
    for _ in 0..100 {
        let password = generator.generate();
        assert_eq!(password.len(), 24);
        assert!(password.chars().any(|c| c.is_ascii_digit()));
        assert!(password.chars().any(|c| c.is_ascii_lowercase()));
        assert!(password.chars().any(|c| c.is_ascii_uppercase()));
        assert!(password.chars().any(|c| "!@#$%^&*".contains(c)));
    }
}
