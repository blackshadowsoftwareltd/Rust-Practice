use crate::book::Book;

pub struct Library {
    books: Vec<Book>,
}
impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }
    pub fn len(&self) -> usize {
        self.books.len()
    }
    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    pub fn print_books(&self) {
        for x in self.books.iter() {
            println!("Book : {:?}", x)
        }
    }
    pub fn oldest_book(&self) -> std::option::Option<Book> {
        if self.is_empty() {
            return None;
        }
        let mut book: Book = self.books[0].clone();
        for i in 1..self.books.len() {
            if self.books[i].published < book.published {
                book = self.books[i].clone()
            }
        }
        Some(book)
    }
}
