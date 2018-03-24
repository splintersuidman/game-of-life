pub fn is_life_105_file<S: AsRef<str>>(s: &S) -> bool {
    s.as_ref().starts_with("#Life 1.05")
}

pub fn parse_life_105_file<S: AsRef<str>>(s: &S) -> Result<super::Pattern, String> {
    let s = s.as_ref();
    let mut pattern = super::Pattern::empty();

    let metadata: String = s.lines()
        .filter(|x| x.starts_with("#D"))
        .map(|x| format!("{}\n", x[2..].trim()))
        .collect();
    
    if metadata.is_empty() {
        pattern.description = None;
    } else {
        pattern.description = Some(metadata);
    }

    // Remove all lines beginning with "#", except the ones with "#P" because they give information
    // about the blocks.
    let lines = s.lines()
        .filter(|x| !x.starts_with('#') || x.starts_with("#P"));

    let mut y: isize = -1;
    let mut base_x: isize = 0;
    for line in lines {
        y += 1;
        let mut x = base_x;
        if line.starts_with("#P") {
            let mut data = line.split_whitespace().skip(1);

            base_x = match data.next() {
                Some(x) => match x.parse() {
                    Ok(x) => x,
                    Err(e) => return Err(format!("Could not read data for x: {}", e)),
                }
                None => return Err(String::from("Could not find data for x in line starting with `#P` while reading a Life 1.05 file.")),
            };

            y = match data.next() {
                Some(y) => match y.parse() {
                    Ok(y) => y,
                    Err(e) => return Err(format!("Could not read data for y: {}.", e))
                },
                None => return Err(String::from("Could not find data for x in line starting with `#P` while reading a Life 1.05 file.")),
            };
        } else {
            for token in line.chars() {
                match token {
                    // Cell is dead.
                    '.' => {}
                    // Cell is alive.
                    '*' => {
                        pattern.cells.push((x, y));
                    }
                    c => {
                        return Err(format!("Unexpected character `{}` while reading a Life 1.05 file, expected `.` or `*`.", c));
                    }
                }
                x += 1;
            }
        }
    }

    Ok(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_life_105_file() {
        assert!(is_life_105_file(&"#Life 1.05\n#D Name: Some name"));
        assert!(!is_life_105_file(&"#Life 1.06\n#D Name: No name"));
    }

    #[test]
    fn test_correct_file() {
        let file = "#Life 1.05
#D Name: B-52 bomber
#D Author: Noam Elkies
#D A period 104 double-barrelled glider gun. It uses a B-heptomino and emits one
#D  glider every 52 generations.
#D www.conwaylife.com/wiki/index.php?title=B-52_bomber
#N
#P -19 -10
.**
.**.................*
...................*.*............*.*
....................*............*
**.......**.......................*..*
**.*.....**.......................*.*.*
...*.......................*.......*..*
...*.......................**.......**
*..*.................**.....*
.**..................*
.....................***
....................................**
....................................**
.**
*..*
*.*.*................*.*....**.....**
.*..*.................**....**.....**.*
.....*............*...*...............*
..*.*............*.*..................*
..................*................*..*
....................................**
";
        assert!(parse_life_105_file(&file).is_ok())
    }

    #[test]
    fn test_incorrect_file() {
        let file = "#Life 1.05
#D Name: B-52 bomber
#D Author: Noam Elkies
#D A period 104 double-barrelled glider gun. It uses a B-heptomino and emits one
#D  glider every 52 generations.
#D www.conwaylife.com/wiki/index.php?title=B-52_bomber
#N
#P -19 -10
.**
.**.................*
...................*.*............*.*
....................*............*
**.......**.......................*..*
**.*.....**.......................*.*.*
...*.......................*.......*..*
...*.....wrong char......**.......**
*..*.................**.....*
.**..................*
.....................***
....................................**
....................................**
.**
*..*
*.*.*................*.*....**.....**
.*..*.................**....**.....**.*
.....*............*...*...............*
..*.*............*.*..................*
..................*................*..*
....................................**
";
        assert!(parse_life_105_file(&file).is_err());

        let file = "#Life 1.05
#D Name: B-52 bomber
#D Author: Noam Elkies
#D A period 104 double-barrelled glider gun. It uses a B-heptomino and emits one
#D  glider every 52 generations.
#D www.conwaylife.com/wiki/index.php?title=B-52_bomber
#N
#P no x no y
.**
.**.................*
...................*.*............*.*
....................*............*
**.......**.......................*..*
**.*.....**.......................*.*.*
...*.......................*.......*..*
...*.......................**.......**
*..*.................**.....*
.**..................*
.....................***
....................................**
....................................**
.**
*..*
*.*.*................*.*....**.....**
.*..*.................**....**.....**.*
.....*............*...*...............*
..*.*............*.*..................*
..................*................*..*
....................................**
";
        assert!(parse_life_105_file(&file).is_err());
    }
}
