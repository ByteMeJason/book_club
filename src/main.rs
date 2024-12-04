#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_read: bool,
}

impl Book {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn author(&self) -> String {
        self.author.clone()
    }

}

struct BookManager {
    books: Vec<Book>,
}

impl BookManager {
    // Add a new book
    fn add_book(&mut self, title: &str, author: &str) {
        let book = Book {
            title: title.to_string(),
            author: author.to_string(),
            is_read: false,
        };
        self.books.push(book);
        println!("Book added");
    }

    fn mark_as_read(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                book.is_read = true;
                return
            }
        }
    }

    // list all books
    fn list_books(&self) {
        if self.books.is_empty() {
            println!("No books in the list");
        } else {
            for b in &self.books {
                println!("{:?}", b);
            }
        }
    }

}





fn main() {
    let mut catalog = BookManager {
        books: Vec::new()
    };

    catalog.add_book("On The Road", "Jack Karoac");
    catalog.add_book("The Bell Jar", "Sylvia Plath");
    
    println!("Book List:");
    catalog.list_books();

    catalog.mark_as_read("On The Road");

    println!("Updated book list");
    catalog.list_books();

 
}


