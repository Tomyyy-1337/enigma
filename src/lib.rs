//! # Enigma Simulator
//! 
//! This crate provides a simulation of the Enigma machine. It is designes to match the original Enigma machine as closely as possible while being 
//! blazingly fast and easy to use. 
//! The simulation includes the following components:
//! - Rotors (Walzen) with configurable turnover notches (Übertragungskerben)
//! - Ring settings (Ringstellung)
//! - Plugboard (Steckerbrett)
//! - Reflector (Umkehrwalze)
//! - Entry wheel (Eintrittswalze)
//! The number of rotors can be choosen arbitrarily.
//! ```rust
//! use enigma::{Enigma, Walze};
//! 
//! fn main() {    
//!     let mut enigma = Enigma::new([
//!         &Walze::I,
//!         &Walze::IV,
//!         &Walze::III,
//!     ]);
//! 
//!     enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX").unwrap();
//!     enigma.set_ringstellung([16, 26, 8]).unwrap();
//!     enigma.set_walzen_stellung([18, 20, 26]).unwrap();
//! 
//!     let test_string = "EJZLBSYEQPDWDUEEJJOUPSOFLBMUIMGLCSKBKJLYZTEIYTHZLUEUHRRKUZOWBVXFOUIZHYGVDXWQKKSBCPTVMNGUCLTQISSBTNSFGNFZCQSJARCNOSEGWMYCHNODWFGGZCQNHZYFATHTQWKGUNWHOXBWKFNPYAMVFT";
//!     let encoded = enigma.encode(test_string);
//! 
//!     assert_eq!(encoded, "XAACHENXAACHENXISTGERETTETXDURQGEBUENDELTENEINSATZDERHILFSKRAEFTEKONNTEDIEBEDROHUNGABGEWENDETUNDDIERETTUNGDERSTADTGEGENXEINSXAQTXNULLXNULLXUHRSIQERGESTELLTWERDENX");
//! }
//! ```
//! Using the correct settings, the simulation can decode WWII messages encoded with the original Enigma machines.
mod enigma;
pub use enigma::{Enigma};

mod walze;
pub use walze::{Eintrittswalze, Umkehrwalze, Walze};