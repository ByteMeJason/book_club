#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

fn main() {
    let book1 = Book{title: String::from("Dados"), author: String::from("your mom"),};
    
    println!("{:?}", book1);
}


