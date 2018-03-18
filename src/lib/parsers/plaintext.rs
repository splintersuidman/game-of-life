/// TODO: docs.
pub fn parse_plaintext_file<S: ToString>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    let s = s.to_string();
    let lines = s.lines().skip_while(|x: &&str| x.starts_with('!'));

    let mut cells: Vec<(isize, isize)> = Vec::new();

    let mut y: isize = 0;
    for line in lines {
        for (x, token) in line.chars().enumerate() {
            match token {
                'O' => {
                    // Cell is alive here
                    cells.push((x as isize, y));
                }
                '.' => {
                    // Cell is dead here
                }
                a => {
                    return Err(format!("Unexpected character `{}` while reading a plaintext file, expected `O` or `.`.", a));
                }
            }
        }
        y += 1;
    }

    Ok(cells)
}
