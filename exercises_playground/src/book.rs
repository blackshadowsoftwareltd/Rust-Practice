#[derive(Debug, Clone)]
pub struct Book {
    pub name: String,
    pub published: u16,
}

impl Book {
    pub fn new(name: &str, year: u16) -> Book {
        Book {
            name: String::from(name),
            published: year,
        }
    }
}
