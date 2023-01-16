use std::fs;

fn main() {

    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut counter_completely = 0;
    let mut counter_partially = 0;
    for line in contents.lines() {
        let mut iterator = line.splitn(2, ",");

        let first_gnome = iterator.next().unwrap();
    
        let mut first_gnome_iterator = first_gnome.splitn(2, "-");
        let first_gnome_start = first_gnome_iterator.next().unwrap().parse::<u128>().unwrap();
        let first_gnome_end = first_gnome_iterator.next().unwrap().parse::<u128>().unwrap();

        let second_gnome = iterator.next().unwrap();

        let mut second_gnome_iterator = second_gnome.splitn(2, "-");
        let second_gnome_start = second_gnome_iterator.next().unwrap().parse::<u128>().unwrap();
        let second_gnome_end = second_gnome_iterator.next().unwrap().parse::<u128>().unwrap();

        if first_gnome_start <= second_gnome_start && first_gnome_end >= second_gnome_end {
            counter_completely += 1;
            println!(" hit: {}", line);
        } else if second_gnome_start <= first_gnome_start && second_gnome_end >= first_gnome_end {
            counter_completely += 1;
            println!(" hit: {}", line);
        }

        if first_gnome_start <= second_gnome_end && second_gnome_start <= first_gnome_end {
            counter_partially += 1;
        }
   }

   println!("overlaps counter_completely: {}", counter_completely);
   println!("overlaps counter_partially: {}", counter_partially);
}