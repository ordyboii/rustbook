mod company;
mod utils;

fn main() {
    let tables = vec![1, 2, 3, 230];
    let median = utils::median_tables(tables).expect("Provide a valid vector");

    println!("{}", median);

    let mut company = company::Company::new("Burger King");
    company.add_department("office");

    println!("Welcome to {} Employee Management System!", company.name);

    loop {
        println!("\nAvailable departments: {:?}", company.list_departments());

        let department_name =
            utils::read_input("Which department do you want to add an employee to?");

        // Try to get existing department or create new one
        let department = match company.get_department_mut(&department_name) {
            Some(dept) => dept,
            None => {
                let create_new = utils::read_input(&format!(
                    "Department '{}' not found. Create it? (y/n)",
                    department_name
                ));

                match create_new.to_lowercase().as_str() {
                    "y" | "yes" => company.add_department(&department_name),
                    _ => {
                        println!("Skipping...");
                        continue;
                    }
                }
            }
        };

        let employee_name = utils::read_input("What is the name of the employee you want to add?");
        department.add_employee(&employee_name);

        println!(
            "✅ Added '{}' to '{}' department",
            employee_name, department_name
        );
        println!(
            "Department now has {} employees: {:?}",
            department.employee_count(),
            department.list_employees()
        );

        let continue_choice = utils::read_input("Do you want to add another employee? (y/n)");
        match continue_choice.to_lowercase().as_str() {
            "n" | "no" => break,
            _ => continue,
        }
    }

    println!("Final company structure:");

    for (dept_name, dept) in &company.departments {
        println!(
            "  {}: {} employees - {:?}",
            dept_name,
            dept.employee_count(),
            dept.list_employees()
        );
    }
}
