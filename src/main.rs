use std::collections::{hash_map::Entry, HashMap};
use std::{io, vec::Vec};

// TODO: need to sort output and make pretty

fn main() {
    let mut employees = HashMap::<String, Vec<String>>::new();

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
            let dept = String::from(&words[3]);
            let name = String::from(&words[1]);

            match employees.entry(dept) {
                Entry::Vacant(e) => {
                    e.insert(vec![name]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(name);
                }
            }
        } else if words.len() == 2 && words[0] == "Show" {
            for (dept, list) in &employees {
                println!("{} : {:?}", &dept, &list);
            }
        } else {
            println!("I don't understand what you want me to do. Let's try this again\n");
        }
    }
}
