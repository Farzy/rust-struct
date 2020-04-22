struct Foo {
    quax: i32,
    baz: String,
}

fn main() {
    let a = Foo {
        quax: 10,
        baz: String::new(),
    };

    println!("Foo: quax: {}, baz: {}", a.quax, a.baz);
}
