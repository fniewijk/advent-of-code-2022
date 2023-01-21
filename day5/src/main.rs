use std::fs;
use math::round::ceil;
use regex::Regex;

fn main() {

    const FILE_PATH:&'static str = "./src/input.txt";
    let contents:String = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let mut sections = contents.splitn(2, "\n\n");

    let stack_input = sections.next().unwrap().to_string();
    let command_input = sections.next().unwrap().to_string();

    println!("{}", stack_input);

    let stacks = process_stack_input(stack_input);

    let result_stacks = process_commands(stacks, command_input);

    println!("{}", read_stacks(result_stacks));
}

fn process_commands(stack_input: Vec<Vec<String>>, command_input: String) -> Vec<Vec<String>> {

    let mut return_stack: Vec<Vec<String>> = stack_input.clone();
    let commands: Vec<(usize, usize, usize)> = read_commands(command_input);

    println!("process commands amount: {:?}", commands.len());
    println!("{:?}", return_stack.clone());

    let mut index = 0;
    for command in commands {
        return_stack = process_command(return_stack, command);
        index += 1;
        println!("index: {}", index);
        //println!("{:?}", return_stack.clone());
    }

    println!("{:?}", return_stack);
    return_stack
}

fn process_command (stack: Vec<Vec<String>>, command: (usize, usize, usize)) -> Vec<Vec<String>> {

    let mut return_stack = stack.clone();

    let amount = command.0;
    let from = command.1 ;
    let to = command.2;

    let index = return_stack[from].len() - amount;
    println!("stack len and slice index: {}: {}", return_stack[from].len(), index);

    let second_chunk = &return_stack[from][index..];
    let mut second_chunk_vector = second_chunk.to_owned();
    // uncomment the line below to solve for 5B
    second_chunk_vector.reverse();

    return_stack[to].append(&mut second_chunk_vector);
    return_stack[from].drain(index..);

    return_stack
}

fn read_commands(command_input: String) -> Vec<(usize, usize, usize)> {

    let mut return_vector: Vec<(usize,usize,usize)> = Vec::new();
    let lines: Vec<&str> = command_input.split("\n").collect();
    let re = Regex::new(r"^([a-z ]*([1-9][0-9]|[1-9])[a-z ]*([1-9])[a-z ]*([1-9]))$").unwrap();

    for line in lines {
        let captures = re.captures(line).expect("It to work");

        let amount = str::parse::<usize>(captures.get(2).unwrap().as_str()).unwrap();
        let from = str::parse::<usize>(captures.get(3).unwrap().as_str()).unwrap();
        let to = str::parse::<usize>(captures.get(4).unwrap().as_str()).unwrap();
        let capture = (amount, from -1, to -1);
        return_vector.push(capture);
    }

    return_vector
}

fn process_stack_input(stack_input: String) -> Vec<Vec<String>> {

    let mut lines: Vec<&str> = stack_input.split("\n").collect();
    lines.remove(lines.len() - 1);

    let stack_length: usize = ceil(lines.first().expect("Should have been able to read the stack config").len() as f64 / 4.0, 0) as usize;

    println!("stack length: {}", stack_length);

    let mut vectors: Vec<Vec<String>> = Vec::new();

    for _index in 0..stack_length {
        vectors.push(Vec::new())
    }
    
    for line in lines.iter().rev() {

        for index in  0..stack_length {

            let letter = line.get(index*4+1..index*4+2).unwrap().to_string();
            
            if letter != " " {
                println!("letter: {}", letter);
                vectors[index].push(letter)
            }
        }
    }
    vectors
}


fn read_stacks( stacks: Vec<Vec<String>>) -> String {

    let mut return_string = String::new();
    for stack in stacks {
        return_string.push_str(stack.last().expect("Something should have been on the stack"));
    }
    return_string
}