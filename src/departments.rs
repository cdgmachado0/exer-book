use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;


pub fn add_department() {
    let departments = ["sales", "operations", "engineering", "support"];
    let employees = ["alice", "bob", "sally", "john"];
    let mut company_record: HashMap<String, String> = HashMap::new();

    let mut user_input = String::new();

    if let Err(_) = io::stdin().read_line(&mut user_input) {
        println!("error happened");
        return;
    };

    user_input = user_input.to_lowercase();

    if !user_input.starts_with("add")  {
        if !user_input.starts_with("remove") {
            println!("Not a valid action. Valid is either ADD or REMOVE");
            return;
        }
    } 

    let dep_index = user_input.rfind(' ').unwrap();

    for (i, c) in user_input.chars().enumerate() {
        if i == 3 && c == ' ' {
            let no_first_space = &user_input[4..];
            let second_space_i = no_first_space.find(' ').unwrap();
            let name = &no_first_space[..second_space_i];

            for employee in employees {
                if name == employee {
                    let no_second_space = &user_input[second_space_i ..];
                    for dep in departments {
                        if dep == no_second_space {
                            edit_employee(name, dep, &mut company_record, true);
                        }
                    }
                }
            };

        }

        if i == 6 && c == ' ' {

        }

    }
}


fn edit_employee(
    name: &str, 
    department: &str, 
    company_record: &mut HashMap<String, String>,
    add_or_remove: bool
) {
    if add_or_remove {
        company_record
            .entry(name.to_string())
            .or_insert(department.to_string());
    } else {
        //remove
    }


}

