// 当结构体持有引用时，也需要生命周期。

// TODO: 修复关于此结构体的编译器错误。
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
