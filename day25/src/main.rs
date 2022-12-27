use std::fs;

fn main () {
    day25_a();

    //13402424204402442331
    let snafu: String = base5_to_snafu(13402424204402442331);

    println!("snafu > {}", snafu);
}
fn day25_a() {
    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");


    let backpacks: Vec<&str> = contents.split("\n").collect();
    let mut all_total: u128 = 0;

    for backpack in backpacks {
        let mut quintimal_string: String = "".to_string();
        let mut subtract_next_item: u128 = 0;
        let snafu: Vec<&str> = backpack.split("").filter(|&x| !x.is_empty()).collect();
        let snafu_length = snafu.len();

        let mut total:u128 = 0;
        for index in (0..snafu_length).rev() {
            let letter = snafu[index];
            let base:u128 = 5;
            let multiplier = base.pow((snafu_length - index - 1) as u32);

            //println!("loop > {} {}", index, multiplier);

            if letter == "-" {
                let next = 4 - subtract_next_item;
                quintimal_string = next.to_string() + &quintimal_string;

                subtract_next_item = 1;
                total+= next * multiplier;

                //println!("loop - > {} {}", quintimal_string, total);
            } else if letter == "=" {
                let next = 3 - subtract_next_item;
                quintimal_string = next.to_string() + &quintimal_string;

                subtract_next_item = 1;
                total+= next * multiplier;

                //println!("loop = > {} {}", quintimal_string, total);
            } else if subtract_next_item > 0 {
                let mut letter_char = letter.parse::<u128>().unwrap();
                // println!("letter_char > {}", letter_char);
                if letter_char == 0 { letter_char = 5 }
                let adjusted_letter = letter_char - subtract_next_item;

                //println!("quintimal_string > {} {} - {}", adjusted_letter, quintimal_string, adjusted_letter.to_string() + &quintimal_string);
                quintimal_string = adjusted_letter.to_string() + &quintimal_string;
                total+= adjusted_letter * multiplier;

                //println!("letter_char > {} {} {}", quintimal_string, letter_char, subtract_next_item);

                subtract_next_item = 0;
                if letter_char == 5 { subtract_next_item = 1; }
                

                //println!("total special item > {} {} {}", quintimal_string, adjusted_letter, subtract_next_item);
            } else {
                quintimal_string = letter.to_owned() + &quintimal_string;
                total+= letter.parse::<u128>().unwrap() * multiplier;

                //println!("letter.parse::<u128>().unwrap()> {} ", letter.parse::<u128>().unwrap());
                //println!("total else quintimal_string total > {} {}", quintimal_string, total);
            }

            //println!("multiplier> {} ", multiplier);
            //println!("total running > {} ", total);
        }
        println!("> {}", quintimal_string);
        println!("total > {} = {}", backpack, total);
        println!("------------");

        all_total+= total;
    }   
    println!("all_total > {}", all_total);
    
}

// fn to_base5(number: u128) -> u128 {

    
// }



fn base5_to_snafu(base_5: u128) -> String{
    // 124030
    let snafu_input = base_5.to_string();
    let mut quintimal_string: String = "".to_string();

    let snafu: Vec<&str> = snafu_input.split("").filter(|&x| !x.is_empty()).collect();
    let snafu_length = snafu.len();
    let mut increase_next_item: u128 = 0;
    for index in (0..snafu_length).rev() {

        let mut number:u128 = snafu[index].parse::<u128>().unwrap() + increase_next_item;

        if number == 5 { 
            number = number % 5; 
            increase_next_item = 1;
        } else {
            increase_next_item = 0;
        }

        if number == 4 {
            quintimal_string = "-".to_string() + &quintimal_string;
            increase_next_item = 1;
        } else if number == 3 {
            quintimal_string = "=".to_string() + &quintimal_string;
            increase_next_item = 1;
        } else {
            quintimal_string = number.to_string() + &quintimal_string;
            
        }
    }

    if increase_next_item == 1 {
        quintimal_string = increase_next_item.to_string() + &quintimal_string;
    }

    return quintimal_string;
    
}
