use std::{
    collections::HashMap,
    io::{self, Write},
};

enum CommandType {
    Add,
    List,
    ListAll,
}

type Person = String;
type Dept = String;

struct Command {
    ctype: CommandType,
    person_name: Option<Person>,
    dept_name: Option<Dept>,
}

pub fn exercise_hash() {
    println!("commands:");
    println!("\tadd <person_name> <dept_name>");
    println!("\tlist <dept_name>");
    println!("\tall");
    let mut employees = HashMap::<Dept, Vec<Person>>::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let command = parse_input();
        run_command(command, &mut employees);
    }
}

fn parse_input() -> Command {
    let command: Command;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read.");
        let words: Vec<_> = input.trim().split(" ").collect();

        match words[0] {
            "add" => {
                if words.len() < 3 {
                    println!("invalid command");
                    continue;
                }
                command = Command {
                    ctype: CommandType::Add,
                    person_name: Some(words[1].to_string()),
                    dept_name: Some(words[2].to_string()),
                }
            }
            "list" => {
                if words.len() < 2 {
                    println!("invalid command");
                    continue;
                }
                command = Command {
                    ctype: CommandType::List,
                    person_name: None,
                    dept_name: Some(words[1].to_string()),
                }
            }
            "all" => {
                command = Command {
                    ctype: CommandType::ListAll,
                    person_name: None,
                    dept_name: None,
                }
            }
            _ => {
                println!("invalid command");
                continue;
            }
        }
        return command;
    }
}

fn run_command(command: Command, employees: &mut HashMap<Dept, Vec<Person>>) {
    match command.ctype {
        CommandType::Add => {
            employees
                .entry(command.dept_name.unwrap())
                .or_insert(Vec::new())
                .push(command.person_name.unwrap());
        }
        CommandType::List => {
            let dept_name = command.dept_name.unwrap();
            let people = employees.get(&dept_name);
            match people {
                None => println!("No such a Dept."),
                Some(people) => {
                    println!("People in the {} Dept. is below.", &dept_name);
                    for person in people {
                        println!("\t{}", &person);
                    }
                }
            }
        }
        CommandType::ListAll => {
            println!("People in our company is below.");
            for (dept, people) in employees {
                println!("--{} Dept.--", dept);
                people.sort();
                for person in people {
                    println!("\t{}", person);
                }
            }
        }
    }
}
