use enigma::{Enigma, Walze, decypher};

fn main() {    
 let mut enigma = Enigma::new([
        &Walze::I,
        &Walze::IV,
        &Walze::II,
    ]);

    enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX").unwrap();
    enigma.set_ringstellung([16, 26, 8]).unwrap();
    enigma.set_walzen_stellung([18, 20, 26]).unwrap();

    let faust = std::fs::read_to_string("faust").expect("Failed to read faust.txt");
    
    let first_10000_chars: String = faust.chars()
        .map(|c| c.to_ascii_uppercase())
        .filter(|c| c.is_ascii_alphabetic())
        .take(5000)
        .collect();
    
    let encoded = enigma.encode_and_reset(&first_10000_chars).unwrap();

    let mut enigma = decypher(&encoded, &Walze::SAMMLUNG_I_V);
    let decoded = enigma.encode_and_reset(&encoded).unwrap();

    assert_eq!(decoded, first_10000_chars);
    println!("Decoded successfully! The decoded text matches the original input.");    
}