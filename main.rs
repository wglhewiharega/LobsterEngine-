use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

// A simple thread-local pseudo‑random number generator.
// It uses the system clock’s sub‑second nanoseconds as initial seed,
// then applies a linear congruential generator.
thread_local! {
    static RNG: Cell<u32> = {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .subsec_nanos();
        Cell::new(seed)
    };
}

/// Returns a pseudo-random integer in the inclusive range `[min, max]`.
fn random_range(min: u32, max: u32) -> u32 {
    RNG.with(|rng| {
        let mut x = rng.get();
        // LCG parameters: multiplier 1664525, increment 1013904223
        x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        rng.set(x);
        min + (x % (max - min + 1))
    })
}

/// Generates a random chess‑like move and prints it to stdout.
/// The output format:
///   For pieces 1‑4: "<piece_name> <file> <rank>"   (e.g., "king e 4")
///   For pawn (5):   "pawn<pawn_number> <file> <rank>" (e.g., "pawn3 a 2")
fn generate_move() {
    // Rank (1..8) – stored as string immediately (like the original)
    let number_component = random_range(1, 8).to_string();

    // File as integer (1..8), will be converted to letter later
    let letter_num = random_range(1, 8);

    // Piece type: 1=king, 2=elephant, 3=horse, 4=boat, 5=pawn
    let piece_type = random_range(1, 5);

    let piece_str = if piece_type == 5 {
        let pawn_num = random_range(1, 4);
        format!("pawn{}", pawn_num)
    } else {
        match piece_type {
            1 => "king".to_string(),
            2 => "elephant".to_string(),
            3 => "horse".to_string(),
            4 => "boat".to_string(),
            _ => unreachable!(),
        }
    };

    // Convert file integer to letter 'a'..'h'
    let letter_component = match letter_num {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        _ => unreachable!(),
    };

    println!("{} {} {}", piece_str, letter_component, number_component);
}

fn main() {
    generate_move();
}
