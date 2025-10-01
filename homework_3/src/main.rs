struct Student{
    name: String,
    major: String,
}

impl Student{
    fn new(n:String, m:String) -> Student {
        Student{
            name: n,
            major: m,
        }
    }
    fn get_name(&self) -> &String{
        return &self.name
    }
    fn get_major(&self) -> &String{
        return &self.major
    }

}

fn main(){
    let me = Student::new("Fabian".to_string(), "Computer Science".to_string());

    println!("My name is {}", me.get_name());
    println!("My major is {}", me.get_major());
}
