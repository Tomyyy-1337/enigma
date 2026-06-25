use std::ops::Deref;

/// Represents the reflector (Umkehrwalze) of the Enigma machine
pub struct Umkehrwalze {
    mapping: [u8; 26],
    name: &'static str,
}

/// Represents the entry wheel (Eintrittswalze) of the Enigma machine
pub struct Eintrittswalze {
    inner_walze: Umkehrwalze,
    inverse_mapping: [u8; 26],
}

/// Represents a rotor (Walze) of the Enigma machine and its associated turnover notches (Übertragungskerben)
pub struct Walze {
    inner_walze: Eintrittswalze,
    übertragungskerben: u32
}

impl Umkehrwalze {
    pub const UKW_A: Umkehrwalze = Umkehrwalze::new("EJMZALYXVBWFCRQUONTSPIKHGD")
        .with_name("UKW A");
    pub const UKW_B: Umkehrwalze = Umkehrwalze::new("YRUHQSLDPXNGOKMIEBFZCWVJAT")
        .with_name("UKW B");
    pub const UKW_C: Umkehrwalze = Umkehrwalze::new("FVPJIAOYEDRZXWGCTKUQSBNMHL")
        .with_name("UKW C");

    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Creates a new reflector (Umkehrwalze) with the specified mapping.
    /// The mapping should be a string of 26 uppercase letters, where each letter represents the output for the corresponding input letter (A-Z). The mapping must be a valid reflector mapping, meaning that each letter must map to a different letter and the mapping must be symmetric (if A maps to B, then B must map to A).
    pub const fn new(mapping: &str) -> Self {
        let mut arr = [0; 26];
        let bytes = mapping.as_bytes();
        let mut i: usize = 0;
        while i < 26 {
            arr[i] = ((bytes[i] - b'A' + 26 - i as u8) % 26) as u8;
            i += 1;
        }
        Umkehrwalze {
            mapping: arr,
            name: ""
        }
    }

    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub(crate) fn map_char(&self, c: u8) -> u8 {
        (c + self.mapping[c as usize]) % 26
    }
}

impl Eintrittswalze {
    pub const ETW: Eintrittswalze = Eintrittswalze::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .with_name("ETW");

    /// Creates a new entry wheel (Eintrittswalze) with the specified mapping.
    /// The mapping should be a string of 26 uppercase letters, where each letter represents the output for the corresponding input letter (A-Z).
    pub const fn new(mapping: &str) -> Self {
        let inner_walze = Umkehrwalze::new(mapping);
        let mut inverse_arr = [0; 26];
        let bytes = mapping.as_bytes();
        let mut i: usize = 0;
        while i < 26 {
            inverse_arr[(i + inner_walze.mapping[i] as usize) % 26] = ((i as u8 + 26 - (bytes[i] - b'A')) % 26) as u8;
            i += 1;
        }
        Eintrittswalze {
            inner_walze,
            inverse_mapping: inverse_arr,
        }
    }

    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.inner_walze = self.inner_walze.with_name(name);
        self
    }

    pub(crate) fn map_char(&self, c: u8) -> u8 {
        self.inner_walze.map_char(c)
    }
   
    pub(crate) fn inverse_map_char(&self, c: u8) -> u8 {
        (c + self.inverse_mapping[c as usize]) % 26
    }
}

impl Walze {
    pub const SAMMLUNG_I_V: [Walze; 5] = [
        Walze::I,
        Walze::II,
        Walze::III,
        Walze::IV,
        Walze::V,
    ];

    pub const SAMMLUNG_I_VIII: [Walze; 8] = [
        Walze::I,
        Walze::II,
        Walze::III,
        Walze::IV,
        Walze::V,
        Walze::VI,
        Walze::VII,
        Walze::VIII,
    ];

    pub const I: Walze = Walze::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ")
        .mit_übertragungskerbe('Q') // Turn the next rotor when the current rotor moves from Q to R
        .with_name("I");
    pub const II: Walze = Walze::new("AJDKSIRUXBLHWTMCQGZNPYFVOE")
        .mit_übertragungskerbe('E')
        .with_name("II");
    pub const III: Walze = Walze::new("BDFHJLCPRTXVZNYEIWGAKMUSQO")
        .mit_übertragungskerbe('V')
        .with_name("III");
    pub const IV: Walze = Walze::new("ESOVPZJAYQUIRHXLNFTGKDCMWB")
        .mit_übertragungskerbe('J')
        .with_name("IV");
    pub const V: Walze = Walze::new("VZBRGITYUPSDNHLXAWMJQOFECK")
        .mit_übertragungskerbe('Z')
        .with_name("V");
    pub const VI: Walze = Walze::new("JPGVOUMFYQBENHZRDKASXLICTW")
        .mit_übertragungskerbe('Z')
        .mit_übertragungskerbe('M')
        .with_name("VI");
    pub const VII: Walze = Walze::new("NZJHGRCXMYSWBOUFAIVLPEKQDT")
        .mit_übertragungskerbe('Z')
        .mit_übertragungskerbe('M')
        .with_name("VII");
    pub const VIII: Walze = Walze::new("FKQHTLXOCBJSPDZRAMEWNIUYGV")
        .mit_übertragungskerbe('Z')
        .mit_übertragungskerbe('M')
        .with_name("VIII");

    /// Creates a new rotor (Walze) with the specified mapping.
    /// The mapping should be a string of 26 uppercase letters, where each letter represents the output for the corresponding input letter (A-Z).
    pub const fn new(mapping: &str) -> Self {
        Walze {
            inner_walze: Eintrittswalze::new(mapping),
            übertragungskerben: 0
        }
    }

    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.inner_walze = self.inner_walze.with_name(name);
        self
    }

    pub(crate) fn map_char(&self, c: u8, stellung: u8) -> u8 {
        (c + self.inner_walze.inner_walze.mapping[(c + stellung) as usize % 26]) % 26
    }

    pub(crate) fn inverse_map_char(&self, c: u8, stellung: u8) -> u8 {
        (c + self.inner_walze.inverse_mapping[(c + stellung) as usize % 26]) % 26
    }

    /// Adds a turnover notch (Übertragungskerbe) to the rotor at the specified character position. 
    /// For example, if the turnover notch is at 'Q', the next rotor will advance when the current rotor moves from 'Q' to 'R'.
    /// A rotor can one ore more turnover notches, which can be set by calling this method multiple times.
    pub const fn mit_übertragungskerbe(mut self, c: char) -> Self {
        let index = (c as u8 - b'A') as usize;
        self.übertragungskerben |= 1 << index;
        self
    }

    pub(crate) fn is_übertragungskerbe(&self, index: usize) -> bool {
        (self.übertragungskerben & (1 << index)) != 0
    }
}

impl Deref for Walze {
    type Target = Eintrittswalze;

    fn deref(&self) -> &Self::Target {
        &self.inner_walze
    }
}

impl Deref for Eintrittswalze {
    type Target = Umkehrwalze;

    fn deref(&self) -> &Self::Target {
        &self.inner_walze
    }
}