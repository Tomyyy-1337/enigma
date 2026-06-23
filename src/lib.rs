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
//! 
//! Using the correct settings, the simulation can decode WWII messages encoded with the original Enigma machines.
//! 
//! ```rust
//! use enigma::{Eintrittswalze, Enigma, Umkehrwalze, Walze};
//! 
//! let mut enigma = Enigma::new(
//!     &Eintrittswalze::ETW,
//!     [
//!         &Walze::I,
//!         &Walze::IV,
//!         &Walze::III,
//!     ],
//!     &Umkehrwalze::UKW_B
//! );
//!
//! enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX");
//! enigma.set_ringstellung([16, 26, 8]);
//! enigma.set_walzen_stellung([18, 20, 26]);
//!
//! let test_string = "EJZLBSYEQPDWDUEEJJOUPSOFLBMUIMGLCSKBKJLYZTEIYTHZLUEUHRRKUZOWBVXFOUIZHYGVDXWQKKSBCPTVMNGUCLTQISSBTNSFGNFZCQSJARCNOSEGWMYCHNODWFGGZCQNHZYFATHTQWKGUNWHOXBWKFNPYAMVFT";
//! let encoded = enigma.encode(test_string);
//! assert_eq!(encoded, "XAACHENXAACHENXISTGERETTETXDURQGEBUENDELTENEINSATZDERHILFSKRAEFTEKONNTEDIEBEDROHUNGABGEWENDETUNDDIERETTUNGDERSTADTGEGENXEINSXAQTXNULLXNULLXUHRSIQERGESTELLTWERDENX");
//! ```
//! 
mod enigma;
pub use enigma::{Enigma, Walze, Eintrittswalze, Umkehrwalze};