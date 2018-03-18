/// TODO: docs.
pub fn parse_life_106_file<S: ToString>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    let s = s.to_string();
    // Skip first line, because it is the header.
    let lines = s.lines().skip(1);

    let mut cells: Vec<(isize, isize)> = Vec::new();

    for line in lines {
        // Skip empty lines.
        if line.is_empty() {
            continue;
        }

        let mut line_split = line.split_whitespace();
        let x = match line_split.next() {
            None => return Err(format!("could not find x in `{}`", line)),
            Some(v) => v,
        };
        let y = match line_split.next() {
            None => return Err(format!("could not find y in `{}`", line)),
            Some(v) => v,
        };

        let x: isize = match x.trim().parse() {
            Err(_) => return Err(format!("could not parse x as number: `{}`", x)),
            Ok(v) => v,
        };
        let y: isize = match y.trim().parse() {
            Err(_) => return Err(format!("could not parse x as number: `{}`", y)),
            Ok(v) => v,
        };

        cells.push((x, y));
    }

    Ok(cells)
}
