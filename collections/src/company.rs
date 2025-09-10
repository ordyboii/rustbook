use std::collections::HashMap;

#[derive(Debug)]
pub struct Company {
    pub name: String,
    pub departments: HashMap<String, Department>,
}

impl Company {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            departments: HashMap::new(),
        }
    }

    pub fn get_department_mut(&mut self, department: &str) -> Option<&mut Department> {
        self.departments.get_mut(department)
    }

    pub fn add_department(&mut self, name: &str) -> &mut Department {
        self.departments
            .entry(name.to_string())
            .or_insert_with(|| Department::new(vec![]))
    }

    pub fn list_departments(&self) -> Vec<&String> {
        self.departments.keys().collect()
    }
}

#[derive(Debug)]
pub struct Department {
    employees: Vec<String>,
}

impl Department {
    pub fn new(employees: Vec<String>) -> Self {
        Self { employees }
    }

    pub fn add_employee(&mut self, name: &str) {
        self.employees.push(name.to_string());
    }

    pub fn list_employees(&self) -> &Vec<String> {
        &self.employees
    }

    pub fn employee_count(&self) -> usize {
        self.employees.len()
    }
}
