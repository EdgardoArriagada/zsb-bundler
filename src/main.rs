mod bundle_lines;
use bundle_lines::bundle_lines;
use std::sync::OnceLock;

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

fn bundled_files(files: &Vec<String>) -> String {
    let mut bundled_files = files.iter().fold(String::new(), |acc, util| {
        let lines = std::fs::read_to_string(util).unwrap();
        acc + &bundle_lines(lines) + "; "
    });

    bundled_files.truncate(bundled_files.len() - 2);
    bundled_files
}

static HOME_CACHE: OnceLock<String> = OnceLock::new();

fn get_home() -> &'static str {
    HOME_CACHE.get_or_init(|| std::env::var("HOME").unwrap())
}

fn get_dir_files(dir: &str) -> Vec<String> {
    let path = format!("{}/.zsh-spell-book/src/{}", get_home(), dir);

    if !std::path::Path::new(&path).exists() {
        panic!("{} does not exist", path);
    }

    output_files(&path)
}

fn main() {
    let bundled_utils = bundled_files(&get_dir_files("utils"));
    println!("le bundled_utils {}", bundled_utils);
}
