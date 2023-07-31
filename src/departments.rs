use std::collections::HashMap;
use std::io;

const DEPARTMENTS: [&str; 4] = ["sales", "operations", "engineering", "support"];
const EMPLOYEES: [&str; 4] = ["alice", "bob", "sally", "john"];

pub fn add_department() {
    // let departments = ["sales", "operations", "engineering", "support"];
    // let employees = ["alice", "bob", "sally", "john"];
    let mut company_record: HashMap<String, Vec<String>> = HashMap::new();

    
    loop {
        let mut user_input = String::new();
        println!("Please insert an action: [ACTION] [EMPLOYEE] to/from [DEPARTMENT]");

        if let Err(_) = io::stdin().read_line(&mut user_input) {
            println!("error happened");
            return;
        };

        user_input = user_input.trim().to_lowercase();

        if user_input.starts_with("list") {
            let display = Action::Display(String::from("all"), String::from("dep"));
            let (action_or_dep, spaces) = filter_whitespaces(&user_input);

            if spaces == 1 && action_or_dep == "all" {
                match display {
                    Action::Display(all, _) => {
                        if action_or_dep == all {
                            show_employes(&company_record, action_or_dep);
                        } else {
                            //error in display task
                        }
                    },
                    _ => ()
                }
                continue;
            } else if spaces == 2 && DEPARTMENTS.contains(&action_or_dep) {
                //for dep [DEPARTMENT]
                
            } else {
                //error in task provided
            }
        }

        if !user_input.starts_with("add")  {
            if !user_input.starts_with("remove") {
                println!("Not a valid action. Valid is either ADD or REMOVE");
                return;
            }
        } 

        

        for (i, c) in user_input.chars().enumerate() {

            if i == 3 && c == ' ' || i == 6 && c == ' ' {
               
                let action = if i == 3 {
                    Action::Add
                } else {
                    Action::Remove
                };

                let no_first_space = &user_input[i + 1..];
                let second_space_i = no_first_space.find(' ').unwrap();
                let name = &no_first_space[..second_space_i];

                for employee in EMPLOYEES { 
                    if name == employee {
                        let possible_dep = user_input.rfind(' ').unwrap();
                        let possible_dep = user_input[possible_dep + 1..].trim();

                        for dep in DEPARTMENTS {
                            if dep == possible_dep {
                                edit_employee(
                                    name.to_string(), 
                                    dep.to_string(), 
                                    &mut company_record, 
                                    &action
                                );
                            }
                        }
                    }
                };
            }
        }
    }
}


fn edit_employee(
    name: String, 
    department: String, 
    company_record: &mut HashMap<String, Vec<String>>,
    add_or_remove: &Action
) {
    match add_or_remove {
        Action::Add => {
            let msg = format!("^^ Success! {} added to {} ^^", c(&name), c(&department));

            company_record
                .entry(department)
                .and_modify(|e| {
                    if e.contains(&name) {
                        println!("Employee is already in this department");
                        return;
                    } else {
                        e.push(name.clone());
                    }
                })
                .or_insert(vec![name]);

            println!("{msg}");
        },
        Action::Remove => {
            company_record.remove(&name);
            println!("^^ Success! {} removed from {} ^^", c(&name), c(&department));
        },
        _ => (),
    }
}

fn show_employes(company_record: &HashMap<String, Vec<String>>, action_or_dep: &str) {

    // for (department, employees) in company_record {
    //     // if action == "all" {
    //     //     for_sorting.push(employee);
    //     // }


    // }

    if action_or_dep == "all" {
        let mut sorted_dep: Vec<&str> = DEPARTMENTS.to_vec();
        sorted_dep.sort(); 

        for dep in sorted_dep {
            let employees = company_record.get(dep);
            if employees == None { 
                continue; 
            }

            let employees = employees.unwrap();
            println!("{}:", c(&dep.to_string()));
            for employee in employees { 
                println!("{employee}");
            }
        }

    }

    // for dep in DEPARTMENTS.sort() {
    //     company_record.get(dep);
    // }

    
    // println!("Employees:");
    // for element in for_sorting {
    //     println!("{}", element);
    // }
}

fn c(entry: &String) -> String {
    let mut entry_chars = entry.chars();
    let l = entry_chars.next().unwrap();
    format!("{}{}", l.to_uppercase(), &entry[1..])
}

fn filter_whitespaces(input: &String) -> (&str, usize) {
    let spaces = input.chars().filter(|c| c.is_whitespace()).count();
    let mut i = 0;
    let mut mut_input: &str = input;

    loop {
        let space_i = mut_input.find(' ').unwrap();
        mut_input = &mut_input[space_i + 1..];
        i += 1;
        if i == spaces {
            return (mut_input, spaces);
        }
    };
}

enum Action {
    Add,
    Remove,
    Display(String, String),
}



