mod bundle_lines;
use bundle_lines::bundle_lines;

fn main() {
    let home = std::env::var("HOME").unwrap();

    let path = format!("{}/.zsh-spell-book/development.zsh", home);

    let lines = std::fs::read_to_string(path).unwrap();

    let result = bundle_lines(lines);

    println!("-------------------");

    println!("{}", result);
}
