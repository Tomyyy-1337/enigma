mod enigma;
use enigma::{Enigma, RotatingWalze};

use crate::enigma::BasicWalze;

fn main() {    
    let mut enigma = Enigma::new(
        &BasicWalze::ETW,
        [
            &RotatingWalze::I,
            &RotatingWalze::IV,
            &RotatingWalze::III,
        ],
        &BasicWalze::UKW_B
    );
    // .mit_steckbrettverbindung('A', 'D')
    // .mit_steckbrettverbindung('C', 'N')
    // .mit_steckbrettverbindung('E', 'T')
    // .mit_steckbrettverbindung('F', 'L')
    // .mit_steckbrettverbindung('G', 'I')
    // .mit_steckbrettverbindung('J', 'V')
    // .mit_steckbrettverbindung('K', 'Z')
    // .mit_steckbrettverbindung('P', 'U')
    // .mit_steckbrettverbindung('Q', 'Y')
    // .mit_steckbrettverbindung('W', 'X');

    enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX");
    enigma.set_ringstellung([16, 26, 8]);
    enigma.set_walzen_stellung([18, 20, 26]);


    let test_string = "EJZLBSYEQPDWDUEEJJOUPSOFLBMUIMGLCSKBKJLYZTEIYTHZLUEUHRRKUZOWBVXFOUIZHYGVDXWQKKSBCPTVMNGUCLTQISSBTNSFGNFZCQSJARCNOSEGWMYCHNODWFGGZCQNHZYFATHTQWKGUNWHOXBWKFNPYAMVFT";
    let encoded = enigma.encode(test_string);
    println!("Encoded: {}", &encoded);

    assert_eq!(encoded, "XAACHENXAACHENXISTGERETTETXDURQGEBUENDELTENEINSATZDERHILFSKRAEFTEKONNTEDIEBEDROHUNGABGEWENDETUNDDIERETTUNGDERSTADTGEGENXEINSXAQTXNULLXNULLXUHRSIQERGESTELLTWERDENX");

}