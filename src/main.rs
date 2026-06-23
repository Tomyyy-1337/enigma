use enigma::{Enigma, Walze};

fn main() {    
    let mut enigma = Enigma::new([
        &Walze::I,
        &Walze::IV,
        &Walze::III,
    ]);

    enigma.set_plugboard("AD CN ET FL GI JV KZ PU QY WX").unwrap();
    enigma.set_ringstellung([16, 26, 8]).unwrap();
    enigma.set_walzen_stellung([18, 20, 26]).unwrap();

    let test_string = "EJZLBSYEQPDWDUEEJJOUPSOFLBMUIMGLCSKBKJLYZTEIYTHZLUEUHRRKUZOWBVXFOUIZHYGVDXWQKKSBCPTVMNGUCLTQISSBTNSFGNFZCQSJARCNOSEGWMYCHNODWFGGZCQNHZYFATHTQWKGUNWHOXBWKFNPYAMVFT";
    let encoded = enigma.encode(test_string);
    println!("Encoded: {}", &encoded);

    assert_eq!(encoded, "XAACHENXAACHENXISTGERETTETXDURQGEBUENDELTENEINSATZDERHILFSKRAEFTEKONNTEDIEBEDROHUNGABGEWENDETUNDDIERETTUNGDERSTADTGEGENXEINSXAQTXNULLXNULLXUHRSIQERGESTELLTWERDENX");
}
