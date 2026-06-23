/// A struct representing an Enigma machine with a configurable number of rotors (N_WALZEN).
pub struct Enigma<const N_WALZEN: usize> {
    eintrittswalze: &'static Eintrittswalze,
    walzen: [&'static Walze; N_WALZEN],
    umkehrwalze: &'static Umkehrwalze,
    ringstellung: [u8; N_WALZEN],
    walzen_stellung: [u8; N_WALZEN],
    steckbrett: [u8; 26],
} 

impl<const N_WALZEN: usize> Enigma<N_WALZEN> {
    pub fn new(
        eintrittswalze: &'static Eintrittswalze,
        walzen: [&'static Walze; N_WALZEN],
        umkehrwalze: &'static Umkehrwalze, 
    ) -> Self {
        let mut steckbrett: [u8; 26] = [0; 26];
        for i in 0..26 {
            steckbrett[i] = i as u8;
        }
        Enigma {
            eintrittswalze,
            walzen,
            umkehrwalze,
            ringstellung: [0; N_WALZEN],
            walzen_stellung: [0; N_WALZEN],
            steckbrett
        }
    }

    pub fn reset_plugboard(&mut self) {
        for i in 0..26 {
            self.steckbrett[i] = i as u8;
        }
    }

    pub fn set_plugboard(&mut self, connections: &str) {
        self.reset_plugboard(); 
        connections.split_ascii_whitespace()
            .for_each(|pair| {
                let [a, b, ..] = pair.as_bytes() else {
                    panic!("Ungültige Steckbrettverbindung: {}", pair);
                };
                let a = (a - b'A') as usize;
                let b = (b - b'A') as usize;
                if self.steckbrett[a] != a as u8 || self.steckbrett[b] != b as u8 {
                    panic!("Überlappende Steckbrettverbindung: {} <-> {}", char::from(a as u8 + b'A'), char::from(b as u8 + b'A'));
                }
                self.steckbrett[a] = b as u8;
                self.steckbrett[b] = a as u8;
            });
    } 

    pub fn encode(&mut self, input: &str) -> String {
        input.chars()
            .map(|c| self.encode_char(c))
            .collect()
    }

    pub fn set_ringstellung(&mut self, mut ringstellung: [u8; N_WALZEN]) {
        for i in 0..N_WALZEN {
            if ringstellung[i] > 26 || ringstellung[i] == 0 {
                panic!("Ungültige Ringstellung: {}", self.ringstellung[i]);
            }
            ringstellung[i] -= 1; 
        }
        self.ringstellung = ringstellung;
    }

    pub fn set_walzen_stellung(&mut self, mut stellungen: [u8; N_WALZEN]) {
        for i in 0..N_WALZEN {
            if stellungen[i] > 26 || stellungen[i] == 0 {
                panic!("Ungültige Walzenstellung: {}", self.walzen_stellung[i]);
            }
            stellungen[i] = (stellungen[i] - 1 + 26 - self.ringstellung[i]) % 26;
        }
        self.walzen_stellung = stellungen;
    }

    pub fn get_walzen_stellung(&self) -> [u8; N_WALZEN] {
        let mut stellungen = [0; N_WALZEN];
        for i in 0..N_WALZEN {
            stellungen[i] = (self.walzen_stellung[i] + self.ringstellung[i]) % 26 + 1;
        }
        stellungen
    }

    fn encode_char(&mut self, c: char) -> char {
        self.increment_walzen_stellung();
        
        let mut c = c as u8 - b'A';
        c = self.steckbrett[c as usize];
        c = self.eintrittswalze.map_char(c);

        for (walze, stellung) in self.walzen.iter().zip(self.walzen_stellung).rev() {
            c = walze.map_char(c, stellung);
        }
        c = self.umkehrwalze.map_char(c);
        for (walze, stellung) in self.walzen.iter().zip(self.walzen_stellung) {
            c = walze.inverse_map_char(c, stellung);
        }
        
        c = self.eintrittswalze.inverse_map_char(c);
        c = self.steckbrett[c as usize];
        (c + b'A') as char
    }

    fn increment_walzen_stellung(&mut self) {
        for i in (0..self.walzen_stellung.len()).rev() {
            let tmp = (self.walzen_stellung[i] + self.ringstellung[i]) % 26;
            self.walzen_stellung[i] = (self.walzen_stellung[i] + 1) % 26;
            if !self.walzen[i].is_übertragungskerbe((tmp) as usize) {
                break;
            }
        }
    }
}

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

    fn map_char(&self, c: u8) -> u8 {
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
    pub const UKW_A: Eintrittswalze = Eintrittswalze::new("EJMZALYXVBWFCRQUONTSPIKHGD");
    pub const UKW_B: Eintrittswalze = Eintrittswalze::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    pub const UKW_C: Eintrittswalze = Eintrittswalze::new("FVPJIAOYEDRZXWGCTKUQSBNMHL");

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

    fn map_char(&self, c: u8) -> u8 {
        self.inner_walze.map_char(c)
    }
   
    fn inverse_map_char(&self, c: u8) -> u8 {
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

    pub fn map_char(&self, c: u8, stellung: u8) -> u8 {
        (self.inner_walze.inner_walze.mapping[(c + stellung) as usize % 26] + c) % 26
    }

    pub fn inverse_map_char(&self, c: u8, stellung: u8) -> u8 {
        (self.inner_walze.inverse_mapping[(c + stellung) as usize % 26] + c) % 26
    }

    pub const fn mit_übertragungskerbe(mut self, c: char) -> Self {
        let index = (c as u8 - b'A') as usize;
        self.übertragungskerben |= 1 << index;
        self
    }

    fn is_übertragungskerbe(&self, index: usize) -> bool {
        (self.übertragungskerben & (1 << index)) != 0
    }
}
