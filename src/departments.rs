use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::io;


pub fn add_department() {
    let departments = ["sales", "operations", "engineering", "support"];
    let employees = ["alice", "bob", "sally", "john"];
    let mut company_record: HashMap<String, String> = HashMap::new();

    
    loop {
        let mut user_input = String::new();
        println!("Please insert an action");

        if let Err(_) = io::stdin().read_line(&mut user_input) {
            println!("error happened");
            return;
        };

        user_input = user_input.trim().to_lowercase();

        if user_input == "list" {
            show_employes(&company_record);
            continue;
        }

        if !user_input.starts_with("add")  {
            if !user_input.starts_with("remove") {
                println!("Not a valid action. Valid is either ADD or REMOVE");
                return;
            }
        } 

        

        for (i, c) in user_input.chars().enumerate() {
            if i == 3 && c == ' ' {
                let no_first_space = &user_input[4..];
                let second_space_i = no_first_space.find(' ').unwrap();
                let name = &no_first_space[..second_space_i];

                for employee in employees { 
                    if name == employee {
                        let possible_dep = user_input.rfind(' ').unwrap();
                        let possible_dep = user_input[possible_dep + 1..].trim();

                        for dep in departments {
                            if dep == possible_dep {
                                edit_employee(
                                    name.to_string(), 
                                    dep.to_string(), 
                                    &mut company_record, 
                                    Action::Add
                                );
                            }
                        }
                    }
                };
            }

            if i == 6 && c == ' ' {

            }

        }
    }
}


fn edit_employee(
    name: String, 
    department: String, 
    company_record: &mut HashMap<String, String>,
    add_or_remove: Action
) {
    match add_or_remove {
        Action::Add => {
            company_record
                .entry(name)
                .or_insert(department);
            println!("Success! {name} added to {department}");
        },
        Action::Remove => {
            //code
        },
    }
}

fn show_employes(company_record: &HashMap<String, String>) {
    for (employee, department) in company_record {
        println!("hi");
        println!("{} - {}", employee, department);
    }
}

fn triage_action(
    i: u8, 
    c: char, 
    user_input: &String, 
    add_or_remove: Action,
    employees: &[&str],
    departments: &[&str],
    company_record: &mut HashMap<String, String>,
) {
    let index = match add_or_remove {
        Action::Add => 3,
        Action::Remove => 6,
    };

    if i == index && c == ' ' {
        let no_first_space = user_input[index + 1..];
        let second_space_i = no_first_space.find(' ').unwrap();
        let name = &no_first_space[..second_space_i];

        for employee in employees { 
            if name == employee {
                let possible_dep = user_input.rfind(' ').unwrap();
                let possible_dep = user_input[possible_dep + 1..].trim();

                for dep in departments {
                    if dep == &possible_dep {
                        edit_employee(
                            name.to_string(), 
                            dep.to_string(), 
                            &mut company_record, 
                            add_or_remove
                        );
                    }
                }
            }
        };
    }

}

enum Action {
    Add,
    Remove
}

