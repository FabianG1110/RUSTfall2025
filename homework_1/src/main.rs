// //Assignment #1
// const FREEZING_POINT_FAHRENHEIT: f64 = 32.0;

// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f - FREEZING_POINT_FAHRENHEIT) * (5.0 / 9.0)
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     (c * (9.0 / 5.0)) + FREEZING_POINT_FAHRENHEIT
// }

// fn main() {
//     let mut temp_f: i32 = 92;
//     let c = fahrenheit_to_celsius(temp_f as f64);
//     println!("{}°F = {:.2}°C", temp_f, c);

//     for _ in 0..5{
//         temp_f += 1;
//         let c = fahrenheit_to_celsius(temp_f as f64);
//         println!("{}°F = {:.2}°C", temp_f, c);

//     }
//      println!("C -> F:");
    
//     for _ in 0..6{
//         let c = fahrenheit_to_celsius(temp_f as f64);
//         let f = celsius_to_fahrenheit(c);
//         println!("{:.2}°C -> {:.0}°F", c, f);
//         temp_f -=1
//     }


// }






// //Assignment #2
// fn is_even(n: i32) -> bool {
//     n % 2 == 0
// }

// fn main(){
//     let numbers = [3, 5, 10, 15, 22, 33, 40, 55, 60, 77];

//     for &num in numbers.iter() {
//         if num % 3 == 0 && num % 5 == 0 {
//             println!("{}: FizzBuzz", num);
//         } else if num % 3 == 0 {
//             println!("{}: Fizz", num);
//         } else if num % 5 == 0 {
//             println!("{}: Buzz", num);
//         } else if is_even(num) {
//             println!("{}: Even", num);
//         } else {
//             println!("{}: Odd", num);
//         }
//     }

//     let mut sum = 0;
//     let mut i = 0;
//     while i < numbers.len() {
//         sum += numbers[i];
//         i += 1;
//     }
//     println!("Sum of all numbers: {}", sum);

//     let mut largest = numbers[0];
//     let mut j = 0;
//     loop {
//         if numbers[j] > largest {
//             largest = numbers[j];
//         }
//         j += 1;
//         if j >= numbers.len() {
//             break;
//         }
//     }
//     println!("Largest number: {}", largest);
// }


//Assignment #3
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main(){
    let secret = 22;
    let mut attempts = 0;
    let mut guess = 10;

    loop {
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0{
            println!("Guess {} is correct!", guess);
            break;
        }else if result == 1{
            println!("Guess {} is too high", guess);
            guess -= 1;
        }else{
            println!("Guess {} is too low", guess);
            guess += 1;
        } 
    }
    println!("It took {} guesses to find the secret!", attempts);
}

