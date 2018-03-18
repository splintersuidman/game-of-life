/// TODO: docs.
pub fn parse_life_105_file<S: ToString>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    let s = s.to_string();

    // Remove all lines beginning with "#", except the ones with "#P" because they give information about the blocks
    let lines = s.lines()
        .filter(|x| !x.starts_with('#') || x.starts_with("#P"));

    let mut cells: Vec<(isize, isize)> = Vec::new();
    let mut y: isize = -1;
    let mut base_x: isize = 0;
    for line in lines {
        y += 1;
        let mut x = base_x;
        if line.starts_with("#P") {
            let mut data = line.split_whitespace().skip(1);

            base_x = match data.next() {
                Some(x) => x.parse().expect("Could not read data for x"),
                None => return Err(format!("Could not find data for x in line starting with `#P` while reading a Life 1.05 file.")),
            };

            y = match data.next() {
                Some(x) => x.parse().expect("Could not read data for x"),
                None => return Err(format!("Could not find data for x in line starting with `#P` while reading a Life 1.05 file.")),
            };
        } else {
            for token in line.chars() {
                match token {
                    '.' => {
                        // Cell is dead here
                    }
                    '*' => {
                        // Cell is alive here
                        cells.push((x, y));
                    }
                    c => {
                        return Err(format!("Unexpected character `{}` while reading a Life 1.05 file, expected `.` or `*`.", c));
                    }
                }
                x += 1;
            }
        }
    }

    Ok(cells)
}
