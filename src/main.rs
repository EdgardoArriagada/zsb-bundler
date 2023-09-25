mod bundle_lines;
use bundle_lines::bundle_lines;

// return all the files in a given dir recursively
fn output_files(dir: &str) -> Vec<String> {
    let paths = std::fs::read_dir(dir).unwrap();
    let mut result = vec![];

    for path in paths {
        let path = path.unwrap().path();

        if path.is_dir() {
            let files = output_files(path.to_str().unwrap());
            for file in files {
                result.push(file);
            }
        } else {
            result.push(path.to_str().unwrap().to_string());
        }
    }

    result
}

fn main() {
    let home = std::env::var("HOME").unwrap();

    let dir = format!("{}/.zsh-spell-book/src/utils", home);
    let utils = output_files(&dir);
    println!("{:?}", utils);

    // let lines = std::fs::read_to_string(dir).unwrap();
    //
    // let result = bundle_lines(lines);
    //
    // println!("-------------------");
    //
    // println!("{}", result);
}
