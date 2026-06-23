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
mod enigma;
pub use enigma::{Enigma, Walze, Eintrittswalze, Umkehrwalze};