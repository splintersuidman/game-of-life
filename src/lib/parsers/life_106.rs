pub fn is_life_106_file<S: AsRef<str>>(s: &S) -> bool {
    s.as_ref().starts_with("#Life 1.06")
}

pub fn parse_life_106_file<S: AsRef<str>>(s: &S) -> Result<super::Pattern, String> {
    let s = s.as_ref();

    // Skip first line, because it is the header.
    let lines = s.lines().skip(1);

    let mut pattern = super::Pattern::empty();

    for line in lines.filter(|s| !s.is_empty()) {
        let mut line_split = line.split_whitespace();

        let x = match line_split.next() {
            None => return Err(format!("could not find x in `{}`", line)),
            Some(v) => v,
        };
        let y = match line_split.next() {
            None => return Err(format!("could not find y in `{}`", line)),
            Some(v) => v,
        };

        let x = match x.trim().parse() {
            Err(_) => return Err(format!("could not parse x as number: `{}`", x)),
            Ok(v) => v,
        };
        let y = match y.trim().parse() {
            Err(_) => return Err(format!("could not parse x as number: `{}`", y)),
            Ok(v) => v,
        };

        pattern.cells.push((x, y));
    }

    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_life_106_file() {
        assert!(is_life_106_file(&"#Life 1.06\n5 0"));
        assert!(!is_life_106_file(&"#Life 1.05\n5 0"));
    }

    #[test]
    fn test_correct_file() {
        let file = "#Life 1.06\n-5 0\n6 7";
        assert!(parse_life_106_file(&file).is_ok())
    }

    #[test]
    fn test_incorrect_file() {
        let file = "#Life 1.06\n-a b\nc d";
        assert!(parse_life_106_file(&file).is_err());
        let file = "#Life 1.06\na b\nc d";
        assert!(parse_life_106_file(&file).is_err());
    }
}
