use enigma::{Enigma, Walze};

fn main() {    
    let mut enigma = Enigma::new(
        [
            &Walze::I,
            &Walze::IV,
            &Walze::III,
        ],
        [16, 26, 8]
    );

    enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX");
    enigma.set_walzen_stellung([18, 20, 26]);

    let test_string = "EJZLBSYEQPDWDUEEJJOUPSOFLBMUIMGLCSKBKJLYZTEIYTHZLUEUHRRKUZOWBVXFOUIZHYGVDXWQKKSBCPTVMNGUCLTQISSBTNSFGNFZCQSJARCNOSEGWMYCHNODWFGGZCQNHZYFATHTQWKGUNWHOXBWKFNPYAMVFT";
    let encoded = enigma.encode(test_string);
    println!("Encoded: {}", &encoded);

    assert_eq!(encoded, "XAACHENXAACHENXISTGERETTETXDURQGEBUENDELTENEINSATZDERHILFSKRAEFTEKONNTEDIEBEDROHUNGABGEWENDETUNDDIERETTUNGDERSTADTGEGENXEINSXAQTXNULLXNULLXUHRSIQERGESTELLTWERDENX");
}