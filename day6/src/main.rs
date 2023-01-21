use std::fs;
use std::collections::VecDeque;

fn main() {
    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let marker_amount = 14;
    let mut index: u64 = 0;
    let mut result = 0;
    let mut letter_bitmasks:VecDeque<u64> = VecDeque::new();
    for letter in contents.chars() {
        let letter_bitmask = get_mask_for_letter(letter);

        letter_bitmasks.push_back(letter_bitmask);

        let score = get_score_from_queue(letter_bitmasks.clone());
        // println!("score: {}", score);
        if score == marker_amount {
            // exit for

            result = index + 1;
            break;
        }

        if index >= marker_amount - 1 {
            let _first_letter_bitmask = letter_bitmasks.pop_front()
                .expect("Expect to be able to get a letter from the front of the queue");
        }
        
        // println!("index: {} : {}", index, result);
        index += 1;
    }

    println!("result: {}", result);

}

fn get_score_from_queue (letter_bitmasks: VecDeque<u64>) -> u64 {
    let mut bitmask = 0;
    // println!("letter_bitmasks {:?}", letter_bitmasks);
    for letter_bitmask in letter_bitmasks.iter() {
        bitmask |= letter_bitmask;

        //println!("bitmask {} letter_bitmask {} ", bitmask, letter_bitmask);
    }
    // println!("get_score {} {} ", bitmask, get_score(bitmask));
    get_score(bitmask)
}

fn get_score (bitmask: u64) -> u64 {
    let mut total:u64 = 0;
    for bit_index  in 0..52 {
        if (bitmask >> bit_index) % 2 == 1 {
            total += 1;
        }
    }
    return total;
}

fn get_mask_for_letter (letter: char) -> u64 {
    let base: u64 = 2;

    // a=97, A=39
    let delta: i32 = -97;
    
    let letter_value_adjusted:i32 = letter as i32 + delta;
    return base.pow(letter_value_adjusted as u32);
}