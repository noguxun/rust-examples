mod parse_json;

fn kick(title: &str, f: fn()) {
    println!("====================");
    println!{" {}", title};
    println!("====================");

    f();

    println!("\n");
}

fn main() {
    kick("Parse JSON", parse_json::run);
}
