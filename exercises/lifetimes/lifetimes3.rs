// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // 将字符串的引用传递给 `Book` 结构体的 `author` 和 `title` 字段
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
