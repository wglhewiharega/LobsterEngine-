use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};


thread_local! {
    static RNG: Cell<u32> = {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .subsec_nanos();
        Cell::new(seed)
    };
}


fn random_range(min: u32, max: u32) -> u32 {
    RNG.with(|rng| {
        let mut x = rng.get();
    
        x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        rng.set(x);
        min + (x % (max - min + 1))
    })
}


fn generate_move() {
 
    let number_component = random_range(1, 8).to_string();

   
    let letter_num = random_range(1, 8);

   
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
