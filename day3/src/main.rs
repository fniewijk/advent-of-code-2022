use std::fs;
use std::str::Split;

fn main() {
    three_a();
    
    three_b();
}

fn three_b() {
    const FILE_PATH:&'static str = "./src/input2.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let backpacks: Vec<&str> = contents.split("\n").collect();
    let gnomes = backpacks.len();
    let mut total: i32 = 0;
    for index in 0..gnomes {

        if index % 3 == 0 {
            let bitmask_one: u64 = get_bitmask(backpacks[index]);
            let bitmask_two: u64 = get_bitmask(backpacks[index + 1]);
            let bitmask_three: u64 = get_bitmask(backpacks[index + 2]);

            let bitmask_total = bitmask_one & bitmask_two & bitmask_three;
            
            total += get_score(bitmask_total);
        }
    }
    println!("> Result B: {:?}", total);
}

fn get_bitmask (backpack: &str) -> u64 {
    let base: u64 = 2;
    let mut bitmask: u64 = 0;
    for item in backpack.split("").filter(|&x| !x.is_empty()) {

            let letter_char = item.parse::<char>().unwrap();
            let letter_value = letter_char as u32;

            // a=97, A=39
            let delta: i64 = if letter_value >= 97 {-97} else {-39};
            
            let letter_value_adjusted:i64 = letter_value as i64 + delta;

            bitmask |= base.pow(letter_value_adjusted as u32);
        }
    return bitmask;
}

fn get_score (bitmask: u64) -> i32 {
    let mut total:i32 = 0;
    let mut letter_value = 1;
    for bit_index  in 0..52 {
        if (bitmask >> bit_index) % 2 == 1 {
            total += letter_value;
        }

        letter_value = letter_value + 1;
    }
    return total;
}

fn three_a () {
    // Way to solve puzzle. 3A Make a 52 bit mask for each letter a-zA-Z. Both pockets have their own mask.
    // for every letter in the pocket flip the bit in the mask corresponding to that letter.
    // Then AND both masks together. 
    // Duplicate letters are revealed in the resulting mask.
    // Loop through the bit's in the resulting mask to get back the sum of the duplicates.

    // complexity, where lines are l and total chars are n is O(n + l*52)
    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
        
    // println!("> File Contents: {}", contents);

    // Regular Results test
    let backpacks: Split<&str> = contents.split("\n");

    let mut total:i32 = 0;

    for backpack in backpacks {
        // println!("> {}", round);

        let half: i128 = backpack.len() as i128 / 2;
        
        let mut bitmask_left: u64 = 0;
        let mut bitmask_right: u64 = 0;
        
        let base: u64 = 2;
        let mut index = 0;
        for item in backpack.split("").filter(|&x| !x.is_empty()) {

            let letter_char = item.parse::<char>().unwrap();
            let letter_value = letter_char as u32;

            // a=97, A=39
            let delta: i64 = if letter_value >= 97 {-97} else {-39};
            
            let letter_value_adjusted:i64 = letter_value as i64 + delta;

            if index < half {
                bitmask_left |=  base.pow(letter_value_adjusted as u32);
            } else {
                bitmask_right |= base.pow(letter_value_adjusted as u32);
            }

            index += 1;
        }

        let bitmask = bitmask_left & bitmask_right;
        // println!("> bitmask : {:?}", bitmask.to_be_bytes());

        total += get_score(bitmask);
        // println!("> Subtotal : {:?}", total);
    }
    println!("> Result A: {:?}", total);
}