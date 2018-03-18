// Perform GET request
// TODO:
// - Bagaimana cara perform GET request di Rust?
// - Apa ekspektasi returnnya? http response?
// - Bagaimana yang ada sekarang?
// oke.
fn get(url: &str) -> &str {
    return url;
}

fn main() {
    let response = get("https://google.com");
    println!("{:?}", response);
}
