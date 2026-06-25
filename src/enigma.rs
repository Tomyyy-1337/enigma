use std::fmt::Debug;

use crate::{Eintrittswalze, Umkehrwalze, Walze};

/// A struct representing an Enigma machine with a configurable number of rotors (N_WALZEN).
#[derive(Clone)]
pub struct Enigma<const N_WALZEN: usize> {
    eintrittswalze: &'static Eintrittswalze,
    walzen: [&'static Walze; N_WALZEN],
    umkehrwalze: &'static Umkehrwalze,
    ring_stellung: [u8; N_WALZEN],
    walzen_stellung: [u8; N_WALZEN],
    steckbrett: [u8; 26],
} 


pub enum EnigmaError {
    InvalidPlugboardConnection(String),
    OverlappingPlugboardConnection(String),
    InvalidWalzenStellung(u8),
    InvalidRingstellung(u8),
    InvalidCharactersInInput(char),
}

impl Debug for EnigmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnigmaError::InvalidPlugboardConnection(pair) => write!(f, "Invalid plugboard connection: {}. Each connection must be a pair of uppercase letters (A-Z) and cannot contain duplicates.", pair),
            EnigmaError::OverlappingPlugboardConnection(pair) => write!(f, "Overlapping plugboard connection: {}. Each letter can only be connected to one other letter.", pair),
            EnigmaError::InvalidWalzenStellung(stellung) => write!(f, "Invalid Walzenstellung: {}. Each rotor position must be between 1 and 26.", stellung),
            EnigmaError::InvalidRingstellung(stellung) => write!(f, "Invalid Ringstellung: {}. Each ring setting must be between 1 and 26.", stellung),
            EnigmaError::InvalidCharactersInInput(c) => write!(f, "Invalid character in input: {}. Only uppercase letters (A-Z) are allowed.", c),
        }
    }
}

impl<const N_WALZEN: usize> Enigma<N_WALZEN> {
    /// Creates a new Enigma machine with the specified entry wheel, rotors, and reflector.
    /// 
    /// By default, [`Umkehrwalze::UKW_B`] and [`Eintrittswalze::ETW`] are used as the reflector and entry wheel, respectively. Can be overridden with `with_umkehrwalze` and `with_eintrittswalze`.
    pub fn new(walzen: [&'static Walze; N_WALZEN]) -> Self {
        let mut enigma = Enigma {
            eintrittswalze: &Eintrittswalze::ETW,
            walzen,
            umkehrwalze: &Umkehrwalze::UKW_B,
            ring_stellung: [0; N_WALZEN],
            walzen_stellung: [0; N_WALZEN],
            steckbrett: [0; 26],
        };
        enigma.reset_plugboard();
        enigma
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
        self.steckbrett = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];    
    }

    /// Sets the plugboard connections based on a string of space-separated letter pairs.
    /// Each pair of letters represents a connection between those two letters on the plugboard.
    /// 
    /// Example input: "AD CN ET FL GI JV KZ PU QY WX" connects A<->D, C<->N, E<->T, etc.
    pub fn set_plugboard(&mut self, connections: &str) -> Result<(), EnigmaError> {
        self.reset_plugboard(); 
        for pair in connections.split_ascii_whitespace() {
            let [a, b, ..] = pair.as_bytes() else {
                return Err(EnigmaError::InvalidPlugboardConnection(pair.to_string()));
            };
            let a = (a - b'A') as usize;
            let b = (b - b'A') as usize;
            if self.steckbrett[a] != a as u8 || self.steckbrett[b] != b as u8 {
                return Err(EnigmaError::OverlappingPlugboardConnection(pair.to_string()));
            }
            self.steckbrett[a] = b as u8;
            self.steckbrett[b] = a as u8;
        };
        Ok(())
    } 

    pub fn set_plugboard_unchecked(&mut self, connections: &[[char; 2]]) {
        self.reset_plugboard();
        for &[a, b] in connections {
            self.set_plug_unchecked(a, b);
        }
    }

    pub fn set_plug_unchecked(&mut self, a: char, b: char) {
        let a = (a as u8 - b'A') as usize;
        let b = (b as u8 - b'A') as usize;
        self.steckbrett[a] = b as u8;
        self.steckbrett[b] = a as u8;
    }

    pub fn reset_plug_unchecked(&mut self, a: char, b: char) {
        let a = (a as u8 - b'A') as usize;
        let b = (b as u8 - b'A') as usize;
        self.steckbrett[a] = a as u8;
        self.steckbrett[b] = b as u8;
    }

    /// Encodes a string of characters using the Enigma machine's current configuration.
    /// The rotors will advance with each character encoded, simulating the behavior of the original Enigma machine.
    /// 
    /// The encode function changes the rotor positions, so the same input will yield different outputs if encoded multiple times without resetting the rotor positions.
    pub fn encode(&mut self, input: &str) -> Result<String, EnigmaError> {
        if let Some(bad_char) = input.chars().find(|&c| !c.is_ascii_uppercase()) {
            return Err(EnigmaError::InvalidCharactersInInput(bad_char));
        }
        let result = input.chars()
            .map(|c| self.encode_char(c))
            .collect();
        Ok(result)
    }   

    pub fn encode_and_reset(&mut self, input: &str) -> Result<String, EnigmaError> {
        let enigma_before = self.clone();
        let walzen_stellung_before = self.walzen_stellung;
        let result = self.encode(input);
        self.walzen_stellung = walzen_stellung_before;
        result
    }

    /// Sets the rotor positions (Walzenstellung) for each rotor.
    pub fn set_walzen_stellung(&mut self, mut stellungen: [u8; N_WALZEN]) -> Result<(), EnigmaError> {
        for i in 0..N_WALZEN {
            if stellungen[i] > 26 || stellungen[i] == 0 {
                return Err(EnigmaError::InvalidWalzenStellung(stellungen[i]));
            }
            stellungen[i] = (stellungen[i] + 26 - 1) % 26;
        }
        self.walzen_stellung = stellungen;
        Ok(())
    }

    /// Sets the ring settings (Ringstellung) for each rotor.
    pub fn set_ringstellung(&mut self, mut ringstellungen: [u8; N_WALZEN]) -> Result<(), EnigmaError> {
        for i in 0..N_WALZEN {
            if ringstellungen[i] > 26 || ringstellungen[i] == 0 {
                return Err(EnigmaError::InvalidRingstellung(ringstellungen[i]));
            }
            ringstellungen[i] = (ringstellungen[i] + 26 - 1) % 26;
        }
        self.ring_stellung = ringstellungen;
        return Ok(());
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

        for i in (0..N_WALZEN).rev() {
            c = self.walzen[i].map_char(c, (self.walzen_stellung[i] + 26 - self.ring_stellung[i]) % 26);
        }
        c = self.umkehrwalze.map_char(c);
        for i in 0..N_WALZEN {
            c = self.walzen[i].inverse_map_char(c, (self.walzen_stellung[i] + 26 - self.ring_stellung[i]) % 26);
        }
        
        c = self.eintrittswalze.inverse_map_char(c);
        c = self.steckbrett[c as usize];
        (c + b'A') as char
    }

    fn increment_walzen_stellung(&mut self) {
        for i in (0..self.walzen_stellung.len()).rev() {
            let tmp = self.walzen_stellung[i];
            self.walzen_stellung[i] = (self.walzen_stellung[i] + 1) % 26;
            if !self.walzen[i].is_übertragungskerbe((tmp) as usize) {
                break;
            }
        }
    }

    pub fn get_plugboard_mapping(&self) -> Vec<[char; 2]> {
        let mut mapping = Vec::new();
        for i in 0..26 {
            let mapped_char = self.steckbrett[i];
            if mapped_char != i as u8 && i < mapped_char as usize {
                mapping.push([
                    (i as u8 + b'A') as char,
                    (mapped_char + b'A') as char,
                ]);
            }
        }
        mapping
    }

    pub fn is_plug_set(&self, c: char) -> bool {
        let index = (c as u8 - b'A') as usize;
        self.steckbrett[index] != index as u8
    }

    pub fn number_of_plugs_set(&self) -> usize {
        self.steckbrett.iter().enumerate().filter(|(i, mapped)| *i != (**mapped) as usize).count() / 2
    }
}
