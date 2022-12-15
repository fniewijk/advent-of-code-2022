use std::fs;

fn main() {

    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
        
    // println!("> File Contents: {}", contents);

    let calories = contents.split("\n");
    let mut gnomeTotalCalories:Vec<u128> = vec!();
    let mut gnome :u128 = 0;
    for caloriesEntry in calories {
        // println!("> Looping {}", caloriesEntry);
        if caloriesEntry == "" {
            gnomeTotalCalories.push(gnome);
            gnome = 0;
        } else {
            gnome = gnome + caloriesEntry.parse::<u128>().unwrap();
        }
    }
    let joined: String = gnomeTotalCalories.iter().map( |&id| id.to_string() + ",").collect();
    // println!("> Gnomes with total calories: {}", joined);

    gnomeTotalCalories.sort();
    let maxGnome = gnomeTotalCalories.pop().unwrap();
    println!("> maxGnome: {:?}", maxGnome);

    let secondMaxGnome= gnomeTotalCalories.pop().unwrap();
    let thirdMaxGnome = gnomeTotalCalories.pop().unwrap();

    println!("> lastThreeGnomes: {:?}", maxGnome + secondMaxGnome + thirdMaxGnome);

}