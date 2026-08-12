use rand::seq::{IndexedRandom, SliceRandom};

use crate::vault::models::PasswordResult;

// Get these from cfg file at some point
pub struct Charset {
    pub numbers: Vec<char>,
    pub lowercase_letters: Vec<char>,
    pub uppercase_letters: Vec<char>,
    pub symbols: Vec<char>,
    pub similar_chars: Vec<char>,
    pub extended_ascii: Vec<char>,
}

impl Default for Charset {
    fn default() -> Self {
        Self {
            numbers: "1234567890".chars().collect(),
            lowercase_letters: "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            uppercase_letters: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),

            // Chosen for maximum website compatibility.
            // Excludes symbols that are commonly rejected
            // by password policies (e.g. spaces and brackets).
            symbols: "!@#$%^&*_-+=,.?".chars().collect(),
            similar_chars: "il1Lo0O".chars().collect(),
            // Empty by default for maximum website compatibility
            extended_ascii: Vec::new(),
        }
    }
}

pub struct Generator {
    pub length: usize,
    pub numbers: bool,
    pub lowercase_letters: bool,
    pub uppercase_letters: bool,
    pub symbols: bool,
    pub spaces: bool,
    pub exclude_similar_characters: bool,
    pub strict: bool,
    pub charset: Charset,
    pub extended_ascii: bool,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            length: 24,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
            charset: Charset::default(),
            extended_ascii: false,
        }
    }
}
impl Charset {
    pub fn populate_extended_ascii(&mut self) -> &mut Self {
        self.extended_ascii = "àáâãäåæçèéêëìíîïðñòóôõöøùúûüýÿÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖØÙÚÛÜÝ"
            .chars()
            .collect();
        self
    }
}
impl Generator {
    pub fn generate(&self) -> PasswordResult {
        let mut available_chars = Vec::new();
        let mut required_chars = Vec::new();
        let mut rng = rand::rng();

        //helper func to add charsets
        let mut add_group = |enabled: bool, chars: &[char]| {
            if enabled {
                let mut pool: Vec<char> = chars.to_vec();
                if self.exclude_similar_characters {
                    pool.retain(|c| !self.charset.similar_chars.contains(c));
                }
                if let Some(&selected) = pool.choose(&mut rng) {
                    available_chars.extend(pool);
                    if self.strict {
                        required_chars.push(selected);
                    }
                }
            }
        };

        add_group(self.numbers, &self.charset.numbers);
        add_group(self.lowercase_letters, &self.charset.lowercase_letters);
        add_group(self.uppercase_letters, &self.charset.uppercase_letters);
        add_group(self.symbols, &self.charset.symbols);
        add_group(self.extended_ascii, &self.charset.extended_ascii);

        if self.spaces {
            available_chars.push(' ');
            if self.strict {
                required_chars.push(' ');
            }
        }

        available_chars.sort_unstable();
        available_chars.dedup();

        let pool_size = available_chars.len();
        if pool_size == 0 {
            return PasswordResult {
                password: String::new(),
                entropy_bits: 0.0,
                pool_size: 0,
            };
        }

        let remaining_length = if self.strict && required_chars.len() <= self.length {
            self.length - required_chars.len()
        } else if !self.strict {
            self.length
        } else {
            0
        };

        let mut password: Vec<char> = required_chars;
        for _ in 0..remaining_length {
            if let Some(&c) = available_chars.choose(&mut rng) {
                password.push(c);
            }
        }

        password.shuffle(&mut rng);
        let password_str: String = password.into_iter().collect();

        // Standard Shannon entropy calculation based on selected pool size:
        // Entropy = length * log2(pool_size)
        let entropy_bits = (password_str.chars().count() as f64) * (pool_size as f64).log2();

        PasswordResult {
            password: password_str,
            entropy_bits,
            pool_size,
        }
    }
}

// Helper method to create a custom generator
impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_length(&mut self, length: usize) -> &mut Self {
        self.length = length;
        self
    }

    pub fn with_numbers(&mut self, enabled: bool) -> &mut Self {
        self.numbers = enabled;
        self
    }

    pub fn with_lowercase(&mut self, enabled: bool) -> &mut Self {
        self.lowercase_letters = enabled;
        self
    }

    pub fn with_uppercase(&mut self, enabled: bool) -> &mut Self {
        self.uppercase_letters = enabled;
        self
    }

    pub fn with_symbols(&mut self, enabled: bool) -> &mut Self {
        self.symbols = enabled;
        self
    }

    pub fn with_spaces(&mut self, enabled: bool) -> &mut Self {
        self.spaces = enabled;
        self
    }

    pub fn with_exclude_similar(&mut self, enabled: bool) -> &mut Self {
        self.exclude_similar_characters = enabled;
        self
    }

    pub fn with_strict(&mut self, enabled: bool) -> &mut Self {
        self.strict = enabled;
        self
    }

    pub fn with_extended_ascii(&mut self, enabled: bool) -> &mut Self {
        self.extended_ascii = enabled;
        if enabled && self.charset.extended_ascii.is_empty() {
            self.charset.populate_extended_ascii();
        }
        self
    }
}
