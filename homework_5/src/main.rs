// define trait show info

// grad student should have a thesis compnent
// gpa and major will be shared

// create another struct called Enrollment
// inside enrollment store undegrad and grads together
// implement show_info  for all enrolled student

// everywhere use generics and traits, no if or match statement
// program to behavior only
pub trait ShowInfo {
    fn show_info(&self) -> String;
}

pub struct Undergrad {
    pub gpa: String,
    pub major: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) -> String {
        format!("Undergrad | Major: {}, GPA: {}", self.major, self.gpa)
    }
}

pub struct Grad {
    pub gpa: String,
    pub major: String,
    pub thesis: String,
}

impl ShowInfo for Grad {
    fn show_info(&self) -> String {
        format!(
            "Graduate | Major: {}, GPA: {}, Thesis: {}",
            self.major, self.gpa, self.thesis
        )
    }
}

// Generic container for ANY student implementing ShowInfo
pub struct Enrollment<T: ShowInfo> {
    pub student: T,
}

impl<T: ShowInfo> ShowInfo for Enrollment<T> {
    fn show_info(&self) -> String {
        self.student.show_info() // behavior-only polymorphism
    }
}

fn main() {
    let u = Undergrad {
        gpa: "3.5".into(),
        major: "Computer Science".into(),
    };

    let g = Grad {
        gpa: "3.9".into(),
        major: "Data Science".into(),
        thesis: "Neural Nets".into(),
    };

    let e1 = Enrollment { student: u };
    let e2 = Enrollment { student: g };

    println!("{}", e1.show_info());
    println!("{}", e2.show_info());
}