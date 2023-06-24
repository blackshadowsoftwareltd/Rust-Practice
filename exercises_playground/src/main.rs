#![deny(clippy::all)]

mod book;
mod library;
use book::Book;
use library::Library;
fn main() {
    let mut library = Library::new();
    println!("The library is empty: {}", library.is_empty());
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    println!("The library is no longer empty: {}", library.is_empty());
    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.name),
        None => println!("The library is empty!"),
    }
    println!("The library has {} books", library.len());
    library.print_books();
}

