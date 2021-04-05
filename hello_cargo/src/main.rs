fn main() {
    println!("(true && false) || true");
    let bool = (true && false) || true;
    println!("{}", bool);

    println!("(3 + 3) * (14 /2)");
    let number = (3 + 3) * (14 / 2);
    println!("{}", number);

    println!("'hello' + 'world'");
    let hello = "hello ".to_string();
    let world = "world";
    println!("{}", hello + world);

    println!("hello world.slice(6)");
    let text = "hello world";
    println!("{}", &text[6..7]);

    println!("シンボルは存在しない");

    println!("配列型");
    let array = ['zero', 'one', 'two']
}
}
