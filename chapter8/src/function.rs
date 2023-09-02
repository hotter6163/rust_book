pub mod question1 {
    pub fn calc_mean(numbers: &[usize]) -> usize {
        let mut sum = 0;
        for number in numbers {
            sum += number;
        }
        sum / numbers.len()
    }

    pub fn calc_medium(numbers: &[usize]) -> usize {
        let mut copy = numbers.to_vec();
        copy.sort();
        if numbers.len() % 2 == 0 {
            (copy[numbers.len() / 2] + copy[(numbers.len() / 2) - 1]) / 2
        } else {
            copy[numbers.len() / 2]
        }
    }

    pub fn calc_mode(numbers: &[usize]) -> usize {
        let mut copy = numbers.to_vec();
        copy.sort();
        let mut max_number = copy[0];
        let mut max_count = 0;
        let mut cursor = 0;
        for i in 0..copy.len() {
            if copy[i] == copy[cursor] {
                continue;
            }

            if i - cursor > max_count {
                max_number = copy[cursor];
                max_count = i - cursor;
            }
            cursor = i;
        }
        max_number
    }
}

pub mod question2 {
    pub fn pig_latin(text: &str) -> String {
        if ['a', 'i', 'u', 'e', 'o'].contains(&String::from(text).chars().next().unwrap()) {
            String::from(text) + "-hay"
        } else {
            String::from(&text[1..])
                + "-"
                + &String::from(text).chars().next().unwrap().to_string()
                + "ay"
        }
    }
}

pub mod question3 {
    use std::collections::HashMap;

    pub struct Company {
        employee_list: HashMap<String, Vec<String>>,
    }

    impl Company {
        pub fn new() -> Company {
            Company {
                employee_list: HashMap::new(),
            }
        }

        pub fn call(&mut self, command: &str) {
            let splitted_command: Vec<&str> = command.split_whitespace().collect();
            match splitted_command.get(0) {
                Some(&"Add") => {
                    if let Some(person) = splitted_command.get(1) {
                        if let Some(department) = splitted_command.get(3) {
                            let list = self
                                .employee_list
                                .entry(department.to_string())
                                .or_insert(vec![]);
                            list.push(person.to_string())
                        }
                    }
                }
                _ => {}
            }
        }

        pub fn get_department_employees(&self, department: &str) -> Option<&Vec<String>> {
            self.employee_list.get(&String::from(department))
        }

        pub fn get_all_employees(&self) -> Vec<String> {
            let mut all_employees: Vec<String> = Vec::new();
            for employees in self.employee_list.values() {
                all_employees.extend(employees.clone());
            }
            all_employees.sort();
            all_employees
        }
    }
}
