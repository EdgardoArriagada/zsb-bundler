#[derive(Debug)]
enum Context {
    Normal,
    EmptyLine,
    Comment,
    DoubleQuoteString,
    SingleQuoteString,
}

pub fn bundle_lines(lines: String) -> String {
    let mut context = Context::Normal;

    let mut result = String::new();
    let mut prev_char = ' ';

    for line in lines.lines() {
        match context {
            Context::Comment | Context::EmptyLine => context = Context::Normal,
            Context::DoubleQuoteString | Context::SingleQuoteString => result.push('\n'),
            _ => match prev_char {
                ' ' | ';' => {}
                '{' => result.push(' '),
                _ => {
                    result.push(';');
                    result.push(' ');
                }
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
                        context = Context::Comment;
                        if !reached_char {
                            break;
                        }

                        if prev_char == ' ' {
                            result.pop();
                        }

                        result.push(';');
                        result.push(' ');
                        break;
                    }
                    ' ' => {
                        if reached_char && prev_char != ' ' {
                            result.push(c);
                        }
                    }
                    '"' => {
                        reached_char = true;
                        context = Context::DoubleQuoteString;
                        result.push(c);
                    }
                    '\'' => {
                        reached_char = true;
                        context = Context::SingleQuoteString;
                        result.push(c);
                    }
                    _ => {
                        reached_char = true;
                        result.push(c);
                    }
                },
                Context::DoubleQuoteString => match c {
                    '"' => {
                        if prev_char != '\\' {
                            context = Context::Normal;
                        }
                        result.push(c);
                    }
                    _ => result.push(c),
                },
                Context::SingleQuoteString => match c {
                    '\'' => {
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

    use std::sync::OnceLock;

    static DIR_CACHE: OnceLock<String> = OnceLock::new();

    fn get_dir() -> String {
        std::env::current_dir().unwrap().display().to_string()
    }

    pub fn get_dir_cache() -> &'static str {
        DIR_CACHE.get_or_init(get_dir)
    }

    fn get_path(file_name: &str, extension: &str) -> String {
        format!(
            "{}/src/test_utils/{}{}",
            get_dir_cache(),
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

    fn get_bundled(file_name: &str) -> String {
        bundle_lines(get_lines(file_name))
    }

    #[test]
    fn test_basic_function() {
        let bundled = get_bundled("basic_function");

        let expected = get_expected("basic_function");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_print_string() {
        let bundled = get_bundled("print_string");

        let expected = get_expected("print_string");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn fn_with_comments() {
        let bundled = get_bundled("fn_with_comments");

        let expected = get_expected("fn_with_comments");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_multi_line_string() {
        let bundled = get_bundled("multi_line_string");

        let expected = get_expected("multi_line_string");

        assert_eq!(bundled, expected);
    }

    #[test]
    fn test_interpolated_string() {
        let bundled = get_bundled("interpolated_string");

        let expected = get_expected("interpolated_string");

        assert_eq!(bundled, expected);
    }
}
