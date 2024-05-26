use std::{collections::HashMap, io, process::Command};

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
// department or all people in the company by department, sorted alphabetically.
fn main() {
    let mut database: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print_options();
        let option = get_user_input("");
        match option.trim() {
            "1" => {
                handle_add_employee(&mut database);
            }
            "2" => handle_list_all_people(&mut database),
            "3" => handle_list_all_employees_per_department(&mut database),
            "4" => {
                let department = get_user_input("Enter a department");
                handle_list_department_employees(&department, &database);
            }
            "q" | "Q" => break,
            _ => {}
        }
    }
}

fn handle_list_all_employees_per_department(database: &mut HashMap<String, Vec<String>>) {
    print_header("All employees per department");
    for (department, employees) in database {
        employees.sort_by_key(|a| a.trim().to_lowercase());
        employees
            .iter()
            .for_each(|emp| println!("{department}   --   {emp}"));
    }
    enter_any_key();
}

fn handle_add_employee(database: &mut HashMap<String, Vec<String>>) {
    clear_terminal();
    loop {
        let command = get_user_input(
            "Run a command using the following format: \"Add <employee_name> to <department>\"",
        );
        let words = command.trim().split(' ');
        // filter out all empty strings
        let words: Vec<&str> = words.into_iter().filter(|w| !w.trim().is_empty()).collect();
        if words.len() != 4 {
            println!("Incorrect format.");
        } else {
            let employee_name = words[1].trim().to_string();
            let department = words[3].trim().to_string();
            database
                .entry(department)
                .and_modify(|v| v.push(String::from(&employee_name)))
                .or_insert(Vec::from([employee_name]));
            break;
        }
    }
}

fn handle_list_department_employees(department: &str, database: &HashMap<String, Vec<String>>) {
    print_header("Employees in {department} department");
    if let Some(employees) = database.get(department) {
        employees.iter().for_each(|e| println!("{}", e));
    }
    enter_any_key();
}

fn handle_list_all_people(database: &mut HashMap<String, Vec<String>>) {
    print_header("All employees");
    for employees in database.values_mut() {
        employees.sort_by_key(|a| a.trim().to_lowercase());
        employees.iter().for_each(|emp| println!("{}", emp));
    }
    enter_any_key();
}

fn enter_any_key() {
    get_user_input("Press enter to continue");
}

fn get_user_input(write_line: &str) -> String {
    println!("{}", write_line);
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to retrieve user input");
    if user_input.ends_with('\n') {
        user_input[..user_input.len() - 1].to_string()
    } else {
        user_input
    }
}

fn print_options() {
    let display = r#"
    Select an option:

    1. Add an employee to a department
    2. List all people
    3. List all people per department
    4. List all people in a Department
    q. Quit
    "#;
    clear_terminal();
    println!("{display}");
}

fn print_header(header: &str) {
    let border = "======================================";
    println!("{}", header);
    println!("{}", border);
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}
