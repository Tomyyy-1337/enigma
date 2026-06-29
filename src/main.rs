use enigma::{Enigma, Walze, decypher};

fn main() {    
    let encoded_faust: String = std::fs::read_to_string("encoded_faust.txt").expect("Failed to read file");

    let mut enigma = Enigma::new([
        &Walze::I,
        &Walze::IV,
        &Walze::III,
    ]);

    enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX").unwrap();
    enigma.set_ringstellung([16, 3, 8]).unwrap();
    enigma.set_walzen_stellung([18, 20, 26]).unwrap();

    let decoded_faust = enigma.encode_and_reset(&encoded_faust).unwrap();
    
    let faust: String = std::fs::read_to_string("faust")
        .expect("Failed to read faust.txt") 
        .chars()   
        .map(|c| c.to_ascii_uppercase())
        .filter(|c| c.is_ascii_alphabetic())
        .take(7500)
        .collect();

    assert_eq!(decoded_faust, faust);

    
}


fn test_decoding() {
    let mut enigma = Enigma::new([
        &Walze::I,
        &Walze::IV,
        &Walze::III,
    ]);

    enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX").unwrap();
    enigma.set_ringstellung([16, 3, 8]).unwrap();
    enigma.set_walzen_stellung([18, 20, 26]).unwrap();

    let faust: String = std::fs::read_to_string("faust")
        .expect("Failed to read faust.txt") 
        .chars()   
        .map(|c| c.to_ascii_uppercase())
        .filter(|c| c.is_ascii_alphabetic())
        .take(7500)
        .collect();
    
    let encoded = enigma.encode_and_reset(&faust).unwrap();

    let mut enigma = decypher(&encoded, &Walze::SAMMLUNG_I_V);
    let decoded = enigma.encode_and_reset(&encoded).unwrap();

    assert_eq!(decoded, faust);
    println!("Decoded successfully! The decoded text matches the original input.");    
}