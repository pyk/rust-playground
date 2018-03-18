// Display value to screen
fn display(value: &'static str) {
    println!("{:?}", value);
}

fn inspect_str(value: &str) {
    println!("======================");
    println!("value: {:?}", value);
    println!("len: {:?}", value.len());
    println!("is_empty: {:?}", value.is_empty());
    println!("as_bytes: {:?}", value.as_bytes());
    println!("======================");
}

fn main() {

    // String literals: `&'static str`
    let text: &'static str = "some text";
    //    │   │    │    │     └──value
    //    │   │    │    └──type
    //    │   │    └──lifetime
    //    │   └──reference
    //    └──bindings

    // Oke. 
    // Sekarang 
    display(text);

    inspect_str(text);
    inspect_str("hello");
    inspect_str("");
    inspect_str("         ");

    // kenapa argument typenya harus &str ? str kok error?
}
