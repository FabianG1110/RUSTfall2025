//Assignment #1
fn concat_strings(s1: &String, s2: &String) -> String{
    format!("{}{}", s1, s2)
}

//Assignment #2
fn clone_and_modify(s: &String) -> String{
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}

//Assignment #3
fn sum(total: &mut i32, low: i32, high: i32) {
    *total = 0;
    for i in low..=high{
        *total += i;
    }
}
fn main(){
    println!("Assignment 1");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result);

    println!("\nAssignment 2");
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s);
    println!("Modified: {}", modified);

    println!("\nAssignment 3");
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("The sum from 0 to 100 is: {}", total);
}
