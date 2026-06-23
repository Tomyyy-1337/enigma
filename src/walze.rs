
/// Represents the reflector (Umkehrwalze) of the Enigma machine
pub struct Umkehrwalze {
    mapping: [u8; 26]
}

impl Umkehrwalze {
    pub const UKW_A: Umkehrwalze = Umkehrwalze::new("EJMZALYXVBWFCRQUONTSPIKHGD");
    pub const UKW_B: Umkehrwalze = Umkehrwalze::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    pub const UKW_C: Umkehrwalze = Umkehrwalze::new("FVPJIAOYEDRZXWGCTKUQSBNMHL");

    pub const fn new(mapping: &str) -> Self {
        let mut arr = [0; 26];
        let bytes = mapping.as_bytes();
        let mut i: usize = 0;
        while i < 26 {
            arr[i] = ((bytes[i] - b'A' + 26 - i as u8) % 26) as u8;
            i += 1;
        }
        Umkehrwalze {
            mapping: arr
        }
    }

    pub(crate) fn map_char(&self, c: u8) -> u8 {
        (c + self.mapping[c as usize]) % 26
    }
}

/// Represents the entry wheel (Eintrittswalze) of the Enigma machine
pub struct Eintrittswalze {
    inner_walze: Umkehrwalze,
    inverse_mapping: [u8; 26],
}

impl Eintrittswalze {
    pub const ETW: Eintrittswalze = Eintrittswalze::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

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

    pub(crate) fn map_char(&self, c: u8) -> u8 {
        self.inner_walze.map_char(c)
    }
   
    pub(crate) fn inverse_map_char(&self, c: u8) -> u8 {
        (c + self.inverse_mapping[c as usize]) % 26
    }
}

/// Represents a rotor (Walze) of the Enigma machine and its associated turnover notches (Übertragungskerben)
pub struct Walze{
    inner_walze: Eintrittswalze,
    übertragungskerben: u32
}

impl Walze {
    pub const I: Walze = Walze::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ")
        .mit_übertragungskerbe('Q'); // Turn the next rotor when the current rotor moves from Q to R
    pub const II: Walze = Walze::new("AJDKSIRUXBLHWTMCQGZNPYFVOE")
        .mit_übertragungskerbe('E');
    pub const III: Walze = Walze::new("BDFHJLCPRTXVZNYEIWGAKMUSQO")
        .mit_übertragungskerbe('V');
    pub const IV: Walze = Walze::new("ESOVPZJAYQUIRHXLNFTGKDCMWB")
        .mit_übertragungskerbe('J');
    pub const V: Walze = Walze::new("VZBRGITYUPSDNHLXAWMJQOFECK")
        .mit_übertragungskerbe('Z');
    pub const VI: Walze = Walze::new("JPGVOUMFYQBENHZRDKASXLICTW")
        .mit_übertragungskerbe('Z')
        .mit_übertragungskerbe('M');
    pub const VII: Walze = Walze::new("NZJHGRCXMYSWBOUFAIVLPEKQDT")
        .mit_übertragungskerbe('Z')
        .mit_übertragungskerbe('M');
    pub const VIII: Walze = Walze::new("FKQHTLXOCBJSPDZRAMEWNIUYGV")
        .mit_übertragungskerbe('Z')
        .mit_übertragungskerbe('M');

    pub const fn new(mapping: &str) -> Self {
        Walze {
            inner_walze: Eintrittswalze::new(mapping),
            übertragungskerben: 0
        }
    }

    pub(crate) fn map_char(&self, c: u8, stellung: u8) -> u8 {
        (self.inner_walze.inner_walze.mapping[(c + stellung) as usize % 26] + c) % 26
    }

    pub(crate) fn inverse_map_char(&self, c: u8, stellung: u8) -> u8 {
        (self.inner_walze.inverse_mapping[(c + stellung) as usize % 26] + c) % 26
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
