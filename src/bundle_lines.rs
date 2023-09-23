enum Context {
    Normal,
    EmptyLine,
    InComment,
    InString,
}

pub fn bundle_lines(lines: String) -> String {
    let mut context = Context::Normal;

    let mut result = String::new();
    let mut prev_char = ' ';

    for line in lines.lines() {
        match context {
            Context::InComment | Context::EmptyLine => context = Context::Normal,
            Context::InString => {}
            _ => match prev_char {
                ' ' | ';' => {}
                '{' => result.push(' '),
                _ => result.push(';'),
            },
        }

        if line.is_empty() {
            context = Context::EmptyLine;
            continue;
        }

        let mut reached_char = false;

        for c in line.chars() {
            match context {
                Context::Normal => match c {
                    '#' => {
                        context = Context::InComment;
                        break;
                    }
                    ' ' => {
                        if reached_char && prev_char != ' ' {
                            result.push(c);
                        }
                    }
                    '\'' | '"' => {
                        reached_char = true;
                        context = Context::InString;
                        result.push(c);
                    }
                    _ => {
                        reached_char = true;
                        result.push(c);
                    }
                },
                Context::InString => match c {
                    '\'' | '"' => {
                        if prev_char != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                    }
                    _ => result.push(c),
                },
                _ => {}
            }

            prev_char = c;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_path(file_name: &str, extension: &str) -> String {
        let dir = std::env::current_dir().unwrap();
        format!(
            "{}/src/test_utils/{}{}",
            dir.display(),
            file_name,
            extension
        )
    }

    fn get_lines(file_name: &str) -> String {
        std::fs::read_to_string(get_path(file_name, ".zsh")).unwrap()
    }

    fn get_expected(file_name: &str) -> String {
        let mut result = std::fs::read_to_string(get_path(file_name, "_expected.zsh")).unwrap();
        // Remove the last newline
        result.truncate(result.len() - 1);

        result
    }

    #[test]
    fn test_bundle_lines() {
        let result = bundle_lines(get_lines("basic_function"));

        let expected = get_expected("basic_function");

        assert_eq!(result, expected);
    }
}
