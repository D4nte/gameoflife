/// For a space that is populated:
///     Each cell with one or no neighbors dies, as if by solitude.
///     Each cell with four or more neighbors dies, as if by overpopulation.
///     Each cell with two or three neighbors survives.
/// For a space that is empty or unpopulated
///     Each cell with three neighbors becomes populated.

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Populated,
}

impl Cell {
    /// Returns a new populated cell.
    fn populated() -> Self {
        Cell::Populated
    }

    /// Returns true if the cell is empty (unpopulated).
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    /// Returns true if the cell is populated.
    fn is_populated(&self) -> bool {
        matches!(self, Cell::Populated)
    }

    /// Makes a cell empty (unpopulated).
    /// Whether the cell was populated or empty does not affect the behaviour of this function.
    fn die(&mut self) {
        *self = Cell::Empty;
    }

    /// Makes a cell populated.
    /// Whether the cell was populated or empty does not affect the behaviour of this function.
    fn spawn(&mut self) {
        *self = Cell::Populated;
    }
}

struct Grid {
    /// Outer Vector represents columns, inner Vec represents rows
    /// e.g. cells[x][y] returns the cell at column x, row y.
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Copy, Clone)]
struct Column(u16);

#[derive(Debug, Copy, Clone)]
struct Row(u16);

impl Column {
    fn new(column: u16) -> Self {
        Self(column)
    }

    fn usize(&self) -> usize {
        self.0 as usize
    }
}

impl Row {
    fn new(row: u16) -> Self {
        Self(row)
    }

    fn usize(&self) -> usize {
        self.0 as usize
    }
}

impl Grid {
    fn new(columns: Column, rows: Row) -> Self {
        let cells = vec![vec![Cell::default(); rows.usize()]; columns.usize()];
        Self { cells }
    }

    fn cell(&self, column_index: Column, row_index: Row) -> Option<Cell> {
        match self.cells.get(column_index.usize()) {
            Some(row) => row.get(row_index.usize()).copied(),
            None => None,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn default_cell_is_empty() {
        let cell = Cell::default();

        assert!(cell.is_empty());
        assert!(!cell.is_populated());
    }

    #[test]
    fn can_instantiate_populated_cell() {
        let cell = Cell::populated();

        assert!(cell.is_populated());
        assert!(!cell.is_empty());
    }

    #[test]
    fn given_populated_cell_when_it_dies_then_it_is_empty() {
        let mut cell = Cell::populated();
        cell.die();
        assert!(cell.is_empty());
    }

    #[test]
    fn given_empty_cell_when_it_spawns_then_it_is_populated() {
        let mut cell = Cell::default();
        cell.spawn();
        assert!(cell.is_populated());
    }

    prop_compose! {
        fn arbitrary_column()(int in any::<u16>()) -> Column {
            Column(int)
        }
    }

    prop_compose! {
        fn arbitrary_row()(int in any::<u16>()) -> Row {
            Row(int)
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 99, .. ProptestConfig::default()
        })]
        #[test]
        fn instantiate_a_defined_size_grid_with_empty_cells(columns in arbitrary_column(),
                                                            rows in arbitrary_row(),
                                                            x in arbitrary_column(),
                                                            y in arbitrary_row()) {
            let grid = Grid::new(columns, rows);
            let cell = grid.cell(x, y);
            if let Some(cell) = cell {
                assert!(cell.is_empty());
            }
        }
    }
}
