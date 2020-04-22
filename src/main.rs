struct Foo {
    quax: i32,
    baz: String,
    z: Fuz,
}

struct Fuz {
    zed: i32,
}

fn main() {
    let a = Foo {
        quax: 10,
        baz: String::from("Hello, World!"),
        z: Fuz { zed: 4 },
    };

    println!("Foo: quax: {}, baz: {}, z.Fuz.zed: {}", a.quax, a.baz, a.z.zed);
}
