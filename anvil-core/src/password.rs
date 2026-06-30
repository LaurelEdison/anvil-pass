use rand::seq::{IndexedRandom, SliceRandom};

// Get these from cfg file at some point
pub struct Charset {
    pub numbers: Vec<char>,
    pub lowercase_letters: Vec<char>,
    pub uppercase_letters: Vec<char>,
    pub symbols: Vec<char>,
    pub similar_chars: Vec<char>,
}

impl Default for Charset {
    fn default() -> Self {
        Self {
            numbers: "1234567890".chars().collect(),
            lowercase_letters: "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            uppercase_letters: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            symbols: "!@#$%^&*".chars().collect(),
            similar_chars: "il1Lo0O".chars().collect(),
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
        }
    }
}

impl Generator {
    pub fn generate(&self) -> String {
        let mut available_chars = Vec::new();
        let mut required_chars = Vec::new();
        let mut rng = rand::rng();

        // Build character pool and required characters
        if self.numbers {
            let mut nums = self.charset.numbers.clone();
            if self.exclude_similar_characters {
                nums.retain(|c| !self.charset.similar_chars.contains(c));
            }
            available_chars.extend(nums.clone());
            if self.strict {
                required_chars.push(*nums.choose(&mut rng).unwrap_or(&'0'));
            }
        }

        if self.lowercase_letters {
            let mut lower = self.charset.lowercase_letters.clone();
            if self.exclude_similar_characters {
                lower.retain(|c| !self.charset.similar_chars.contains(c));
            }
            available_chars.extend(lower.clone());
            if self.strict {
                required_chars.push(*lower.choose(&mut rng).unwrap_or(&'a'));
            }
        }

        if self.uppercase_letters {
            let mut upper = self.charset.uppercase_letters.clone();
            if self.exclude_similar_characters {
                upper.retain(|c| !self.charset.similar_chars.contains(c));
            }
            available_chars.extend(upper.clone());
            if self.strict {
                required_chars.push(*upper.choose(&mut rng).unwrap_or(&'A'));
            }
        }

        if self.symbols {
            let mut symbols = self.charset.symbols.clone();
            if self.exclude_similar_characters {
                symbols.retain(|c| !self.charset.similar_chars.contains(c));
            }
            available_chars.extend(symbols.clone());
            if self.strict {
                required_chars.push(*symbols.choose(&mut rng).unwrap_or(&'!'));
            }
        }

        if self.spaces {
            available_chars.push(' ');
            if self.strict {
                required_chars.push(' ');
            }
        }

        // Ensure we have at least one character to choose from
        if available_chars.is_empty() {
            return String::new();
        }

        // Calculate remaining length after required characters
        let remaining_length = if self.strict && required_chars.len() <= self.length {
            self.length - required_chars.len()
        } else if !self.strict {
            self.length
        } else {
            // If strict but not enough length for required chars, just use available length
            0
        };

        // Generate remaining characters
        let mut password: Vec<char> = required_chars;

        // Fill the rest with random characters
        for _ in 0..remaining_length {
            if let Some(&c) = available_chars.choose(&mut rng) {
                password.push(c);
            }
        }

        // Shuffle the password to mix required characters throughout
        password.shuffle(&mut rng);

        password.into_iter().collect()
    }
}

// Helper method to create a custom generator
impl Generator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn with_numbers(mut self, enabled: bool) -> Self {
        self.numbers = enabled;
        self
    }

    pub fn with_lowercase(mut self, enabled: bool) -> Self {
        self.lowercase_letters = enabled;
        self
    }

    pub fn with_uppercase(mut self, enabled: bool) -> Self {
        self.uppercase_letters = enabled;
        self
    }

    pub fn with_symbols(mut self, enabled: bool) -> Self {
        self.symbols = enabled;
        self
    }

    pub fn with_spaces(mut self, enabled: bool) -> Self {
        self.spaces = enabled;
        self
    }

    pub fn with_exclude_similar(mut self, enabled: bool) -> Self {
        self.exclude_similar_characters = enabled;
        self
    }

    pub fn with_strict(mut self, enabled: bool) -> Self {
        self.strict = enabled;
        self
    }
}
