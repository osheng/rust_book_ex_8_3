use std::collections::HashMap;
use std::vec::Vec;
use std::io;

fn main() {
    let mut employees = HashMap::new();

    loop {
        println!("What do you want to do?");

        let mut request = String::new();

        io::stdin()
            .read_line(&mut request)
            .expect("Failed to read line! {request}");

        let mut words: Vec<String> = Vec::new();

        for word in request.split_whitespace() {
            words.push(word.to_string());
        }

        if words.len() == 4 && words[0] == "Add" && words[2] == "to" {
            employees.entry(String::from(&words[3])).or_insert(String::from(&words[1]));
        } else if words.len() == 2 && words[0] == "Show" {
            for (dept, name) in &employees {
                println!("{}: {}", dept, name);
            }
        } else {
            println!("I don't understand what you want me to do. Let's try this again\n");
        }
    }
}
