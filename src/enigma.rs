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
    /// Creates a new Enigma machine with the specified entry wheel, rotors, and reflector.
    /// 
    /// By default, [`Umkehrwalze::UKW_B`] and [`Eintrittswalze::ETW`] are used as the reflector and entry wheel, respectively. Can be overridden with `with_umkehrwalze` and `with_eintrittswalze`.
    pub fn new(
        walzen: [&'static Walze; N_WALZEN],
    ) -> Self {
        let mut steckbrett: [u8; 26] = [0; 26];
        for i in 0..26 {
            steckbrett[i] = i as u8;
        }
        Enigma {
            eintrittswalze: &Eintrittswalze::ETW,
            walzen,
            umkehrwalze: &Umkehrwalze::UKW_B,
            ringstellung: [0; N_WALZEN],
            walzen_stellung: [0; N_WALZEN],
            steckbrett
        }
    }

    /// Changes the entry wheel (Eintrittswalze) of the Enigma machine to the specified one.
    pub fn with_eintrittswalze(mut self, eintrittswalze: &'static Eintrittswalze) -> Self {
        self.eintrittswalze = eintrittswalze;
        self
    }

    /// Changes the reflector (Umkehrwalze) of the Enigma machine to the specified one.
    pub fn with_umkehrwalze(mut self, umkehrwalze: &'static Umkehrwalze) -> Self {
        self.umkehrwalze = umkehrwalze;
        self
    }

    /// Unplugs all plugboard connections, resetting the plugboard to its default state where each letter maps to itself.
    pub fn reset_plugboard(&mut self) {
        for i in 0..26 {
            self.steckbrett[i] = i as u8;
        }
    }

    /// Sets the plugboard connections based on a string of space-separated letter pairs.
    /// Each pair of letters represents a connection between those two letters on the plugboard.
    /// 
    /// Example input: "AD CN ET FL GI JV KZ PU QY WX" connects A<->D, C<->N, E<->T, etc.
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

    /// Encodes a string of characters using the Enigma machine's current configuration.
    /// The rotors will advance with each character encoded, simulating the behavior of the original Enigma machine.
    /// 
    /// The encode function changes the rotor positions, so the same input will yield different outputs if encoded multiple times without resetting the rotor positions.
    pub fn encode(&mut self, input: &str) -> String {
        input.chars()
            .map(|c| self.encode_char(c))
            .collect()
    }

    /// Sets the rotor positions (Walzenstellung) for each rotor.
    pub fn set_walzen_stellung(&mut self, mut stellungen: [u8; N_WALZEN]) {
        for i in 0..N_WALZEN {
            if stellungen[i] > 26 || stellungen[i] == 0 {
                panic!("Ungültige Walzenstellung: {}", self.walzen_stellung[i]);
            }
            stellungen[i] = (stellungen[i] + 26 - 1) % 26;
        }
        self.walzen_stellung = stellungen;
    }

    /// Sets the ring settings (Ringstellung) for each rotor.
    pub fn set_ringstellung(&mut self, mut ringstellungen: [u8; N_WALZEN]) {
        for i in 0..N_WALZEN {
            if ringstellungen[i] > 26 || ringstellungen[i] == 0 {
                panic!("Ungültige Ringstellung: {}", self.ringstellung[i]);
            }
            ringstellungen[i] = (ringstellungen[i] + 26 - 1) % 26;
        }
        self.ringstellung = ringstellungen;
    }

    /// Returns the current rotor positions (Walzenstellung) for each rotor, adjusted to a 1-based index (1-26).
    pub fn get_walzen_stellung(&self) -> [u8; N_WALZEN] {
        let mut stellungen = [0; N_WALZEN];
        for i in 0..N_WALZEN {
            stellungen[i] = (self.walzen_stellung[i]) % 26 + 1;
        }
        stellungen
    }

    fn encode_char(&mut self, c: char) -> char {
        self.increment_walzen_stellung();
        
        let mut c = c as u8 - b'A';
        c = self.steckbrett[c as usize];
        c = self.eintrittswalze.map_char(c);

        for ((walze, stellung), ringstellung) in self.walzen.iter().zip(self.walzen_stellung).zip(self.ringstellung).rev() {
            c = walze.map_char(c, (stellung + 26 - ringstellung) % 26);
        }
        c = self.umkehrwalze.map_char(c);
        for ((walze, stellung), ringstellung) in self.walzen.iter().zip(self.walzen_stellung).zip(self.ringstellung) {
            c = walze.inverse_map_char(c, (stellung + 26 - ringstellung) % 26);
        }
        
        c = self.eintrittswalze.inverse_map_char(c);
        c = self.steckbrett[c as usize];
        (c + b'A') as char
    }

    fn increment_walzen_stellung(&mut self) {
        for i in (0..self.walzen_stellung.len()).rev() {
            let tmp = (self.walzen_stellung[i]) % 26;
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

    /// Adds a turnover notch (Übertragungskerbe) to the rotor at the specified character position. 
    /// For example, if the turnover notch is at 'Q', the next rotor will advance when the current rotor moves from 'Q' to 'R'.
    /// A rotor can one ore more turnover notches, which can be set by calling this method multiple times.
    pub const fn mit_übertragungskerbe(mut self, c: char) -> Self {
        let index = (c as u8 - b'A') as usize;
        self.übertragungskerben |= 1 << index;
        self
    }

    fn is_übertragungskerbe(&self, index: usize) -> bool {
        (self.übertragungskerben & (1 << index)) != 0
    }
}
