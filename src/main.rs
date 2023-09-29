mod bundle_lines;
use bundle_lines::bundle_lines;
use std::sync::OnceLock;

fn output_zsh_files(dir: &str) -> Vec<String> {
    let paths = std::fs::read_dir(dir).unwrap();
    let mut result = vec![];

    for path in paths {
        let path = path.unwrap().path();

        if path.is_dir() {
            let files = output_zsh_files(path.to_str().unwrap());
            for file in files {
                if file.ends_with(".zsh") {
                    result.push(file);
                }
            }
        } else {
            let path = path.to_str().unwrap();
            if path.ends_with(".zsh") {
                result.push(path.to_string());
            }
        }
    }

    result
}

static HOME_CACHE: OnceLock<String> = OnceLock::new();

fn get_home() -> &'static str {
    HOME_CACHE.get_or_init(|| std::env::var("HOME").unwrap())
}

fn get_dir_zsh_files(dir: &str) -> Vec<String> {
    let path = format!("{}/.zsh-spell-book/{}", get_home(), dir);

    if !std::path::Path::new(&path).exists() {
        println!("WARNING: {} does not exist", path);
        return vec![];
    }

    output_zsh_files(&path)
}

fn get_file(file: &str) -> String {
    let path = format!("{}/.zsh-spell-book/{}", get_home(), file);

    if !std::path::Path::new(&path).exists() {
        println!("WARNING: {} does not exist", path);
        return String::new();
    }

    path.to_string()
}

fn bundled_zsh_files(dir_name: &str) -> String {
    let files = get_dir_zsh_files(dir_name);

    let mut bundled_files = files.iter().fold(String::new(), |acc, util| {
        let lines = std::fs::read_to_string(util).unwrap();
        acc + &bundle_lines(lines)
    });

    bundled_files.truncate(bundled_files.len() - 2);
    bundled_files
}

fn bundle_file(file_name: &str) -> String {
    let file = get_file(file_name);

    let lines = std::fs::read_to_string(file).unwrap();
    bundle_lines(lines)
}

fn write_result(result: String) {
    let path = format!("{}/.zsh-spell-book/result1.zsh", get_home());
    std::fs::write(path, result).unwrap();
}

fn main() {
    let bundled_env = bundle_file(".env");
    let bundled_zsh_config = bundle_file("src/zsh.config.zsh");
    let bundled_vars = bundle_file("src/globalVariables.zsh");

    let bundled_utils = bundled_zsh_files("src/utils");
    let bundled_configs = bundled_zsh_files("src/configurations");
    let bundled_spells = bundled_zsh_files("src/spells");

    let bundled_temp_spells = bundled_zsh_files("src/temp/spells");
    let bundled_calls = bundled_zsh_files("src/automatic-calls");

    let result = bundled_env
        + "; "
        + &bundled_zsh_config
        + "; "
        + &bundled_vars
        + "; "
        + &bundled_utils
        + "; "
        + &bundled_configs
        + "; "
        + &bundled_spells
        + "; "
        + &bundled_temp_spells
        + "; "
        + &bundled_calls;

    write_result(result);

    println!("Done!");
}
