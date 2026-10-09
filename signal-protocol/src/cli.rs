use std::env;
use std::process;
use signal_protocol::TruthAnchor;

fn print_usage() {
    println!("Codex Babel · Truth Engine CLI");
    println!("Usage:");
    println!("  truth-engine-cli verify <seed> <u> <v> <w> <n> \"<text>\" \"<hash>\"");
    println!("  truth-engine-cli anchor <seed> <u> <v> <w> <n>");
    println!("");
    println!("Examples:");
    println!("  truth-engine-cli anchor 420691337 0 0 0 4");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "anchor" => {
            if args.len() < 7 {
                eprintln!("Error: 'anchor' requires <seed> <u> <v> <w> <n>");
                process::exit(1);
            }
            let seed: u64 = args[2].parse().expect("Invalid seed");
            let u: i64 = args[3].parse().expect("Invalid u");
            let v: i64 = args[4].parse().expect("Invalid v");
            let w: i64 = args[5].parse().expect("Invalid w");
            let n: usize = args[6].parse().expect("Invalid n");

            match babel_core::reconstruct_block(seed, (u, v, w), n) {
                Ok((text, _amps)) => {
                    let anchor = TruthAnchor::new(seed, (u, v, w), n, text.clone());
                    println!("--- TRUTH ANCHOR CERTIFICATE ---");
                    println!("Seed:        {}", anchor.seed);
                    println!("Center (K):  ({}, {}, {})", anchor.center.0, anchor.center.1, anchor.center.2);
                    println!("Dimension:   {}x{}x{}", anchor.n, anchor.n, anchor.n);
                    println!("Text Hash:   {}", anchor.text_hash);
                    println!("Decoded:     \"{}\"", anchor.claimed_text);
                    println!("Verified:    {}", anchor.verify());
                    println!("--------------------------------");
                }
                Err(e) => {
                    eprintln!("Failed to compute 3D IFFT transform: {}", e);
                    process::exit(1);
                }
            }
        }
        "verify" => {
            if args.len() < 9 {
                eprintln!("Error: 'verify' requires <seed> <u> <v> <w> <n> \"<text>\" \"<hash>\"");
                process::exit(1);
            }
            let seed: u64 = args[2].parse().expect("Invalid seed");
            let u: i64 = args[3].parse().expect("Invalid u");
            let v: i64 = args[4].parse().expect("Invalid v");
            let w: i64 = args[5].parse().expect("Invalid w");
            let n: usize = args[6].parse().expect("Invalid n");
            let claimed_text = args[7].clone();
            let claimed_hash = args[8].clone();

            let mut anchor = TruthAnchor::new(seed, (u, v, w), n, claimed_text);
            anchor.text_hash = claimed_hash;

            let is_valid = anchor.verify();
            println!("Verification result: {}", if is_valid { "VALID PROOF (SAT)" } else { "INVALID PROOF (UNSAT)" });
            if !is_valid {
                process::exit(2);
            }
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}
