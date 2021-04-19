use std::collections::HashMap;

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
    let  array: [String; 3] = ["zero".to_string(), "one".to_string(), "two".to_string()];
    println!("{:?}", array);

    println!("range");
    let range = std::ops::Range { start: 3, end: 9 };
    for n in range {
        println!("{:?}", n);
    };

    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");
    println!("{:?}", contacts);
}
