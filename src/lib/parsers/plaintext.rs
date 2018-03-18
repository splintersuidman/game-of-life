pub fn is_plaintext_file<S: AsRef<str>>(s: &S) -> bool {
    s.as_ref().starts_with("!Name:")
}

/// TODO: docs.
pub fn parse_plaintext_file<S: AsRef<str>>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    let s = s.as_ref();

    let lines = s.lines().skip_while(|x| x.starts_with('!'));

    let mut cells = Vec::new();

    for (y, line) in lines.enumerate() {
        for (x, token) in line.chars().enumerate() {
            match token {
                // Cell is alive.
                'O' => {
                    cells.push((x as isize, y as isize));
                }
                // Cell is dead.
                '.' => {}
                a => {
                    return Err(format!("Unexpected character `{}` while reading a plaintext file, expected `O` or `.`.", a));
                }
            }
        }
    }

    Ok(cells)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_correct_file() {
        unimplemented!()
    }

    #[test]
    fn test_incorrect_file() {
        unimplemented!()
    }
}
