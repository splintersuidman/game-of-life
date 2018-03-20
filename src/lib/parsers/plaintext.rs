pub fn is_plaintext_file<S: AsRef<str>>(s: &S) -> bool {
    s.as_ref().starts_with("!Name:")
}

pub fn parse_plaintext_file<S: AsRef<str>>(s: &S) -> Result<super::Pattern, String> {
    let s = s.as_ref();

    let _metadata = s.lines().take_while(|x| x.starts_with('!'));

    // TODO: process metadata

    let lines = s.lines().skip_while(|x| x.starts_with('!'));

    let mut pattern = super::Pattern::empty();

    for (y, line) in lines.enumerate() {
        for (x, token) in line.chars().enumerate() {
            match token {
                // Cell is alive.
                'O' => {
                    pattern.cells.push((x as isize, y as isize));
                }
                // Cell is dead.
                '.' => {}
                a => {
                    return Err(format!("Unexpected character `{}` while reading a plaintext file, expected `O` or `.`.", a));
                }
            }
        }
    }

    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_plaintext_file() {
        assert!(is_plaintext_file(&"!Name: My name"));
        assert!(!is_plaintext_file(&"No name"));
    }

    #[test]
    fn test_correct_file() {
        let file = "!Name: My name\n.O\n..O\nOOO";
        assert!(parse_plaintext_file(&file).is_ok())
    }

    #[test]
    fn test_incorrect_file() {
        let file = "!Name: My name\n.O\n..Owrong characters\nOOO";
        assert!(parse_plaintext_file(&file).is_err())
    }
}
