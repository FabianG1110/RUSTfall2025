use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).expect("Failed to create file");
    for b in books {
        // Simple CSV: title,author,year
        writeln!(file, "{},{},{}", b.title, b.author, b.year)
            .expect("Failed to write to file");
    }

}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Vec::new(), // return empty if file doesn't exist
    };

    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.splitn(3, ','); // title, author, year
            let title = match parts.next() { Some(s) => s.trim().to_string(), None => continue };
            let author = match parts.next() { Some(s) => s.trim().to_string(), None => continue };
            let year_s = match parts.next() { Some(s) => s.trim(), None => continue };
            if let Ok(year) = year_s.parse::<u16>() {
                books.push(Book { title, author, year });
            }
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}