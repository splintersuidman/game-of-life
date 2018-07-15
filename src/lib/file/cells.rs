use super::CellState;

#[derive(Clone)]
pub enum Cells {
    List(CellList),
    Table(CellTable),
}

#[derive(Clone, Default)]
pub struct CellList {
    pub cells: Vec<(isize, isize)>,
    // TODO: RENAME(origin).
    pub center: (isize, isize),
}

impl CellList {
    #[inline]
    pub fn push(&mut self, value: (isize, isize)) {
        self.cells.push(value)
    }
}

// TODO: benchmark.
impl From<CellTable> for CellList {
    fn from(table: CellTable) -> CellList {
        let mut list = CellList::default();
        // TODO: appropriate to say center = (width / 2, height / 2)?
        list.center = (table.width as isize / 2, table.height as isize / 2);

        for y in 0..table.width {
            for x in 0..table.height {
                if table.cells[y][x] == CellState::Alive {
                    list.cells.push((
                        x as isize - list.center.0,
                        y as isize - list.center.1,
                    ));
                }
            }
        }

        list
    }
}

impl From<Cells> for CellList {
    fn from(cells: Cells) -> CellList {
        match cells {
            Cells::List(list) => list,
            Cells::Table(table) => table.into(),
        }
    }
}

impl IntoIterator for CellList {
    type Item = (isize, isize);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}


#[derive(Clone, Default)]
pub struct CellTable {
    pub cells: Vec<Vec<CellState>>,
    pub width: usize,
    pub height: usize,
}

// TODO: benchmark.
impl From<CellList> for CellTable {
    fn from(list: CellList) -> CellTable {
        use std::iter;

        let mut min_x = isize::max_value();
        let mut max_x = isize::min_value();
        let mut min_y = isize::max_value();
        let mut max_y = isize::min_value();

        for &(x, y) in &list.cells {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_x {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        let mut table = CellTable::default();

        table.width = (max_x - min_x) as usize + 1;
        table.height = (max_y - min_y) as usize + 1;

        table.cells = iter::repeat(iter::repeat(CellState::Dead).take(table.width).collect())
            .take(table.height)
            .collect();

        for (x, y) in list.cells {
            table.cells[(y - min_y) as usize][(x - min_x) as usize] = CellState::Alive;
        }

        table
    }
}

impl From<Cells> for CellTable {
    fn from(cells: Cells) -> CellTable {
        match cells {
            Cells::List(list) => list.into(),
            Cells::Table(table) => table,
        }
    }
}

impl IntoIterator for CellTable {
    type Item = Vec<CellState>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}


impl Cells {
    // Convenient because Into<T>::into does not accept type parameters.
    #[inline]
    pub fn into_cell_list(self) -> CellList {
        self.into()
    }

    // Convenient because Into<T>::into does not accept type parameters.
    #[inline]
    pub fn into_cell_table(self) -> CellTable {
        self.into()
    }
}

impl Default for Cells {
    fn default() -> Cells {
        // This won't allocate, so not really a problem.
        Cells::Table(CellTable::default())
    }
}
