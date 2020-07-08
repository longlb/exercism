use std::collections::HashMap;
use std::collections::HashSet;

pub struct School {
    grades: HashSet<u32>,
    students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashSet::new(),
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.insert(grade);
        if let None = self.students.get_mut(&grade) {
            self.students.insert(grade, Vec::new());
        }
        self.students
            .get_mut(&grade)
            .unwrap()
            .push(student.to_string())
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = self.grades.clone().into_iter().collect();
        result.sort();
        result
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.students.get(&grade) {
            Some(x) => {
                let mut result = x.clone();
                result.sort();
                Some(result)
            }
            None => None,
        }
    }
}
