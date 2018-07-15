extern crate game_of_life;
use game_of_life::file::{CellList, CellTable};
use std::collections::HashSet;

#[test]
fn test_list_table() {
    let mut list_first: HashSet<(isize, isize)> = HashSet::new();

    let mut list = CellList::default();
    list.origin = (0, 0);
    list.cells = vec![(0, -1), (1, 0), (-1, 1), (0, 1), (1, 1)];

    for cell in list.cells.iter() {
        list_first.insert(cell.clone());
    }

    let table: CellTable = list.into();
    let list: CellList = table.into();

    for cell in list.cells {
        assert!(list_first.contains(&cell));
    }
}
