use super::{CellList, Cells, Parse, Pattern, Serialise};
use std::fmt;

pub struct Life106;

impl Serialise for Life106 {
    fn serialise<W: fmt::Write>(output: &mut W, pattern: Pattern) -> Result<(), fmt::Error> {
        write!(output, "#Life 1.06")?;

        for c in pattern.cells.into_cell_list().into_iter() {
            write!(output, "\n{} {}", c.0, c.1)?;
        }

        Ok(())
    }
}

impl Parse for Life106 {
    fn is_type<S: AsRef<str>>(file: S) -> bool {
        file.as_ref().starts_with("#Life 1.06")
    }

    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String> {
        let file = file.as_ref();

        // Skip first line, because it is the header.
        let lines = file.lines().skip(1);

        let mut cells = CellList::default();

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

            cells.push((x, y));
        }

        let mut pattern = Pattern::default();
        pattern.cells = Cells::List(cells);

        Ok(pattern)
    }
}
