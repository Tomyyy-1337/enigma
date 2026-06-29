use std::{io::{self, Write}};
use itertools::{Itertools};
use rayon::{iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator}};

use crate::{Enigma, Walze};

pub fn decypher(cyphertext: &str, possible_rotors: &'static [Walze]) -> Enigma<3> {
    println!("Finding possible rotor combinations...");

    let mut best_score = f64::MIN;
    let mut best_enigma = Enigma::new([&possible_rotors[0], &possible_rotors[1], &possible_rotors[2]]);

    let best_rotor_groups = select_walzen(&cyphertext, possible_rotors);

    print!("Rotor Candidates found:");
    for rotors in best_rotor_groups.iter() {
        print!("[{} {} {}] ", possible_rotors[rotors[0]].name(), possible_rotors[rotors[1]].name(), possible_rotors[rotors[2]].name());
    }
    println!("\n---------------------------------------------------------------");

    for rotors in best_rotor_groups.iter() {
        println!("Testing rotor combination: {} {} {}", possible_rotors[rotors[0]].name(), possible_rotors[rotors[1]].name(), possible_rotors[rotors[2]].name());
        
        let rotors = [&possible_rotors[rotors[0]], &possible_rotors[rotors[1]], &possible_rotors[rotors[2]]];
        let rotor_settings = select_first_ringstellung(&cyphertext, rotors);

        let mut best_ring_score = f64::MIN;
        let mut best_ring_setting = (0,0,0,0,0);
        for (ring_3, walze_3) in rotor_settings.into_iter() {
            let (score, (ring_2, walze_1, walze_2)) = select_second_ringstellung(&cyphertext, rotors, ring_3, walze_3);
            if score > best_ring_score {
                best_ring_score = score;
                best_ring_setting = (ring_2, ring_3, walze_1, walze_2, walze_3);
            }
        }
        let (ring_2, ring_3, walze_1, walze_2, walze_3) = best_ring_setting;
        println!("Ringstellung: [1, {ring_2}, {ring_3}], Walzenstellung: [{walze_1}, {walze_2}, {walze_3}]");
        
        let plugboard = solve_pluggboard(
            &cyphertext, 
            rotors, 
            [1, ring_2, ring_3],
            [walze_1, walze_2, walze_3]
        );

        let mut enigma = Enigma::new([rotors[0], rotors[1], rotors[2]]);
        enigma.set_ringstellung([1, ring_2, ring_3]).unwrap();  
        enigma.set_walzen_stellung([walze_1, walze_2, walze_3]).unwrap();
        for [a, b] in &plugboard {
            enigma.set_plug_unchecked(*a, *b);    
        }

        let decoded = enigma.encode_and_reset(&cyphertext).unwrap();
        let score = score_german(&decoded);

        println!("Plugboard: {:?}\nScore: {:.4}", plugboard, score);
        println!("---------------------------------------------------------------");

        if score > best_score {
            best_score = score;
            best_enigma = enigma;
        }

    }

    let walzen = best_enigma.get_walzen_stellung();
    let ringstellung = best_enigma.get_ringstellung();
    let plugboard = best_enigma.get_plugboard_mapping();

    println!("===============================================================");
    println!("Solution found with Rotors: {} {} {}", best_enigma.get_rotors()[0].name(), best_enigma.get_rotors()[1].name(), best_enigma.get_rotors()[2].name());
    println!("Ringstellung: [{}, {}, {}]", ringstellung[0], ringstellung[1], ringstellung[2]);
    println!("Walzenstellung: [{}, {}, {}]", walzen[0], walzen[1], walzen[2]);
    println!("Plugboard: {:?}", plugboard);
    println!("===============================================================");

    best_enigma
}

pub fn solve_pluggboard(
    cyphertext: &str,
    walzen_selection: [&'static Walze; 3],
    ringstellung: [u8; 3],
    walzenstellung: [u8; 3],
) -> Vec<[char; 2]> {
    let mut enigma = Enigma::new([
        walzen_selection[0],
        walzen_selection[1],
        walzen_selection[2],
    ]);
    enigma.set_ringstellung(ringstellung).unwrap();
    enigma.set_walzen_stellung(walzenstellung).unwrap();

    let mut best_score = score_german(&enigma.encode_and_reset(cyphertext).unwrap());
    let mut best_plugboard = Vec::new();
    
    let tuple_candidates = ('A'..='Z')
        .array_combinations::<2>()
        .collect::<Vec<_>>();
    
    let mut todo = vec![PlugboardState {
        enigma,
        next_plug_index: 0,
        score: 0.0
    }];
    let mut new_todo = Vec::new();

    for _ in 0..13 {
        if todo.is_empty() {
            break;
        }
        print!("\rPlugboard: {:?}", best_plugboard);
        io::stdout().flush().unwrap();
        
        let best_score_at_loop_start = best_score;
        while let Some(PlugboardState { mut enigma, next_plug_index, .. }) = todo.pop() {
            for c in &tuple_candidates {
                if enigma.is_plug_set(c[0]) || enigma.is_plug_set(c[1]) {
                    continue;
                }
                
                enigma.set_plug_unchecked(c[0], c[1]);
                
                let decoded = enigma.encode_and_reset(cyphertext).unwrap();
                let score = score_german(&decoded);
                
                if score > best_score {
                    best_score = score;
                    best_plugboard = enigma.get_plugboard_mapping();
                }
                
                if score > best_score_at_loop_start {
                    new_todo.push(PlugboardState {
                        enigma: enigma.clone(),
                        next_plug_index: next_plug_index + 1,
                        score
                    });
                } 

                enigma.reset_plug_unchecked(c[0], c[1]);
            }
        }
        print!("\r\x1b[2K");
        io::stdout().flush().unwrap();

        todo = new_todo;
        new_todo = Vec::new();   
        
        if todo.len() > 50 {
            todo.select_nth_unstable_by(10, |a, b| b.score.partial_cmp(&a.score).unwrap());
            todo.truncate(50);
        } 
    }
        
    best_plugboard
}

struct PlugboardState {
    enigma: Enigma<3>,
    next_plug_index: usize,
    score: f64,
}

fn select_first_ringstellung(cyphertext: &str, walzen_selection: [&'static Walze; 3]) -> Vec<(u8, u8)> {   
    let started = std::sync::atomic::AtomicUsize::new(0);
    let finished = std::sync::atomic::AtomicUsize::new(0);
    
    let ceil = 26*26;
    let scores: Vec<_> = (0..ceil).into_par_iter().map(|l| {
        print!("\rCalculating scores for Walzen. [Started: {:>3} Finished: {:>3} Total: {ceil}]", started.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1, finished.load(std::sync::atomic::Ordering::SeqCst));
        io::stdout().flush().unwrap();
        let mut enigma = Enigma::new([
            walzen_selection[0],
            walzen_selection[1],
            walzen_selection[2],
        ]);
        
        let mut max_score = 0.0;
        let mut best_walzen_stellung = (0,0);
        let a = (l / 26) as u8 + 1;
        let b = (l % 26) as u8 + 1;
        enigma.set_ringstellung([1, 1, a]).unwrap();
        for j in 0..26 {
            for k in 0..26 {
                enigma.set_walzen_stellung([k + 1, j + 1, b]).unwrap();
                let decoded = enigma.encode(cyphertext).unwrap();
                let score = score_text(&decoded);
                if score > max_score {
                    max_score = score;
                    best_walzen_stellung = (a, b);
                }
            }
        }
        print!("\rCalculating scores for Walzen. [Started: {:>3} Finished: {:>3} Total: {ceil}]", started.load(std::sync::atomic::Ordering::SeqCst), finished.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1);
        io::stdout().flush().unwrap();
        (max_score, best_walzen_stellung)
    }).collect();


    print!("\r\x1b[2K");
    io::stdout().flush().unwrap();

    scores.into_iter()
        .sorted_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap())
        .take(5)
        .map(|(_, walzen_stellung)| walzen_stellung)
        .collect()
}

fn select_second_ringstellung(
    cyphertext: &str, 
    walzen_selection: [&'static Walze; 3],
    ringstellung: u8,
    walzenstellung: u8,
) -> (f64, (u8, u8, u8)) {   
    let started = std::sync::atomic::AtomicUsize::new(0);
    let finished = std::sync::atomic::AtomicUsize::new(0);
    
    let ceil = 26*26;
    let scores: Vec<_> = (0..ceil).into_par_iter().map(|l| {
        print!("\rCalculating scores for Walzen. [Started: {:>3} Finished: {:>3} Total: {ceil}]", started.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1, finished.load(std::sync::atomic::Ordering::SeqCst));
        io::stdout().flush().unwrap();
        let mut enigma = Enigma::new([
            walzen_selection[0],
            walzen_selection[1],
            walzen_selection[2],
        ]);
        
        let mut max_score = 0.0;
        let mut best_walzen_stellung = (0,0,0);
        let a = (l / 26) as u8 + 1;
        let b = (l % 26) as u8 + 1;
        enigma.set_ringstellung([1, a, ringstellung]).unwrap();
        for j in 0..26 {
            enigma.set_walzen_stellung([j + 1, b, walzenstellung]).unwrap();
            let decoded = enigma.encode(cyphertext).unwrap();
            let score = score_text(&decoded);
            if score > max_score {
                max_score = score;
                best_walzen_stellung = (a, j + 1, b);
            }
        }
        print!("\rCalculating scores for Walzen. [Started: {:>3} Finished: {:>3} Total: {ceil}]", started.load(std::sync::atomic::Ordering::SeqCst), finished.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1);
        io::stdout().flush().unwrap();
        (max_score, best_walzen_stellung)
    }).collect();


    print!("\r\x1b[2K");
    io::stdout().flush().unwrap();

    scores.into_iter()
        .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn select_walzen(cyphertext: &str, walzen: &'static[Walze]) -> Vec<[usize; 3]> {
    let permutations = (0..walzen.len())
        .permutations(3)
        .collect::<Vec<_>>();

    // println!("Total walzen combinations to test: {}. Resulting in {} combinations with 3 rotors.", permutations.len(), permutations.len() * 26 * 26 * 26);
    // println!("Calculating scores for all walzen combinations with all possible walzen_stellung (rotor positions)...");

    let started = std::sync::atomic::AtomicUsize::new(0);
    let finished = std::sync::atomic::AtomicUsize::new(0);

    let walzen_scores = permutations.par_iter()
        .map(|p| {
            print!("\rCalculating scores for walzen combinations. [Started: {:>3} Finished: {:>3} Total: {:>3}]", started.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1, finished.load(std::sync::atomic::Ordering::SeqCst), permutations.len());
            io::stdout().flush().unwrap();
            let walzen = [&walzen[p[0]], &walzen[p[1]], &walzen[p[2]]];
            let mut enigma = Enigma::new(walzen);
            let mut max_score = 0.0;
            for i in 0..26 {
                for j in 0..26 {
                    for k in 0..26 {
                        enigma.set_walzen_stellung([i + 1, j + 1, k + 1]).unwrap();
                        let decoded = enigma.encode(cyphertext).unwrap();
                        let score = score_text(&decoded);
                        if score > max_score {
                            max_score = score;
                        }
                    }
                }
            }
            print!("\rCalculating scores for walzen combinations. [Started: {:>3} Finished: {:>3} Total: {:>3}]", started.load(std::sync::atomic::Ordering::SeqCst), finished.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1, permutations.len());
            ((p[0], p[1], p[2]), max_score)
        })
        .collect::<Vec<_>>();

    let best_score = walzen_scores.iter().map(|(_, score)| *score).fold(f64::MIN, f64::max);

    print!("\r\x1b[2K");
    io::stdout().flush().unwrap();

    walzen_scores.into_iter()
        .filter(|(_, score)| *score >= best_score * 0.991) 
        .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
        .map(|(walzen, _)| [walzen.0, walzen.1, walzen.2])
        .take(3)
        .collect()
}

pub fn score_text(text: &str) -> f64 {
    let (entropy_score, ic) = entropy_ic(text);
    let repeeted_letters = repeeted_letters_score(text);
    ic + entropy_score + repeeted_letters
}

const GERMAN_LETTER_FREQUENCIES: [f64; 26] = [
    6.51, 1.89, 3.06, 5.08, 17.40, 1.66, 3.01, 4.76, 7.55, 0.27, 1.21, 3.44,
    2.53, 9.78, 2.51, 0.79, 0.02, 7.00, 7.27, 6.15, 4.35, 0.67, 1.89, 0.03,
    0.04, 1.13,
];

fn score_german(text: &str) -> f64 {
    let mut letter_frequencies = [0; 26];
    
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let index = (c.to_ascii_uppercase() as u8 - b'A') as usize;
            letter_frequencies[index] += 1;
        }
    }

    let total_letters = text.chars().filter(|c| c.is_ascii_alphabetic()).count() as f64;
    let mut score = 0.0;
    for i in 0..26 {
        let observed_frequency = letter_frequencies[i] as f64 / total_letters;
        let expected_frequency = GERMAN_LETTER_FREQUENCIES[i] / 100.0;
        score += (observed_frequency - expected_frequency) * (observed_frequency - expected_frequency);
    }

    1.0 - score
}

fn repeeted_letters_score(text: &str) -> f64 {
    let count = text.as_bytes().windows(2).filter(|window| window[0] == window[1]).count();
    let score = count as f64 / (text.len() as f64 - 1.0);
    0.3 - (0.027 - score).abs()
}

fn entropy_ic(text: &str) -> (f64, f64) {
    let mut counts = [0; 26];
    let total = text.len();

    for c in text.chars() {
        let index = (c.to_ascii_uppercase() as u8 - b'A') as usize;
        counts[index] += 1;
    }

    let mut entropy = 0.0;
    for count in counts.iter() {
        if *count > 0 {
            let p = *count as f64 / total as f64;
            entropy -= p * p.log2();
        }
    }
    entropy = 1.0 - (entropy - 4.1).abs(); 

    let mut ic = 0.0;
    for count in counts.iter() {
        ic += (*count as f64 * (*count as f64 - 1.0)) / (total as f64 * (total as f64 - 1.0));
    }
    ic *= 10.0;

    (entropy, ic)
}
