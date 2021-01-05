/// For a space that is populated:
///     Each cell with one or no neighbors dies, as if by solitude.
///     Each cell with four or more neighbors dies, as if by overpopulation.
///     Each cell with two or three neighbors survives.
/// For a space that is empty or unpopulated
///     Each cell with three neighbors becomes populated.
use anyhow::{bail, Result};
use std::ops::Add;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Column(u16);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Row(u16);

impl Column {
    fn new(column: u16) -> Self {
        Self(column)
    }

    fn usize(&self) -> usize {
        self.0 as usize
    }

    fn try_sub(self, rh: u16) -> Option<Self> {
        let lh = self.0;

        if lh > rh {
            Some(Self(lh - rh))
        } else {
            None
        }
    }
}

impl Row {
    fn new(row: u16) -> Self {
        Self(row)
    }

    fn usize(&self) -> usize {
        self.0 as usize
    }

    fn try_sub(self, rh: u16) -> Option<Self> {
        let lh = self.0;

        if lh > rh {
            Some(Self(lh - rh))
        } else {
            None
        }
    }
}

impl Add<u16> for Column {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<u16> for Row {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self(self.0 + rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    column: Column,
    row: Row,
}

impl Coordinates {
    fn new(column: Column, row: Row) -> Self {
        Self { column, row }
    }

    /// Returns the neighbouring coordinates.
    /// If the coordinates are near a 0 edges then the invalid
    /// neighbours are omitted.
    /// (i-1,j-1) (i,j-1)   (i+1,j-1)
    /// (i-1,j)   Self(i,j) (i+1,j)
    /// (i-1,j+1) (i,j+1)   (i+1,j+1)

    fn neighbours(&self) -> Vec<Coordinates> {
        let mut neighbours = vec![];

        if let (Some(column), Some(row)) = (self.column.try_sub(1), self.row.try_sub(1)) {
            neighbours.push(Coordinates::new(column, row));
        }
        if let (Some(column), row) = (self.column.try_sub(1), self.row) {
            neighbours.push(Coordinates::new(column, row));
        }
        if let (Some(column), row) = (self.column.try_sub(1), self.row + 1) {
            neighbours.push(Coordinates::new(column, row));
        }
        neighbours.push(Coordinates::new(self.column, self.row + 1));
        neighbours.push(Coordinates::new(self.column + 1, self.row + 1));
        neighbours.push(Coordinates::new(self.column + 1, self.row));
        if let (column, Some(row)) = (self.column + 1, self.row.try_sub(1)) {
            neighbours.push(Coordinates::new(column, row));
        }
        if let (column, Some(row)) = (self.column, self.row.try_sub(1)) {
            neighbours.push(Coordinates::new(column, row));
        }
        neighbours
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

    fn populate(&mut self, column_index: Column, row_index: Row) -> Result<()> {
        match self.cells.get_mut(column_index.usize()) {
            Some(row) => match row.get_mut(row_index.usize()) {
                Some(cell) => {
                    cell.spawn();
                    Ok(())
                }
                None => bail!("Coordinates are out of bound."),
            },
            None => bail!("Coordinates are out of bound."),
        }
    }

    fn neighbour_cells(&self, column_index: Column, row_index: Row) -> Vec<Cell> {
        let coordinates = Coordinates::new(column_index, row_index).neighbours();
        let mut neighbours = vec![];

        for coordinates in coordinates {
            if let Some(cell) = self.cell(coordinates.column, coordinates.row) {
                neighbours.push(cell)
            }
        }

        neighbours
    }

    fn next(&mut self) {
        for i in 0..self.cells.len() {
            // TODO: have a safe method on grid
            for j in 0..self.cells[i].len() {
                let column = Column::new(i as u16);
                let row = Row::new(j as u16);
                let neighbours = self.neighbour_cells(column, row);
                let mut populated = 0;
                for cell in neighbours {
                    if cell.is_populated() {
                        populated += 1;
                    }
                }

                if populated < 2 {
                    self.cells[i][j].die()
                }
            }
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

    #[test]
    fn given_new_grid_when_populate_specific_cells_then_they_are_populated() {
        let mut grid = Grid::new(Column::new(20), Row::new(20));

        let coordinates_1 = (Column::new(2), Row::new(3));
        let coordinates_2 = (Column::new(5), Row::new(12));
        grid.populate(coordinates_1.0, coordinates_1.1).unwrap();
        grid.populate(coordinates_2.0, coordinates_2.1).unwrap();

        let cell_1 = grid
            .cell(coordinates_1.0, coordinates_1.1)
            .expect("some cell");
        let cell_2 = grid
            .cell(coordinates_2.0, coordinates_2.1)
            .expect("some cell");

        assert!(cell_1.is_populated());
        assert!(cell_2.is_populated());
    }

    #[test]
    fn given_coordinate_when_neighbours_requested_return_them() {
        let coordinate = Coordinates::new(Column::new(5), Row::new(3));

        let neighbours = coordinate.neighbours();

        let expected_neighbours = vec![
            Coordinates::new(Column::new(4), Row::new(2)),
            Coordinates::new(Column::new(4), Row::new(3)),
            Coordinates::new(Column::new(4), Row::new(4)),
            Coordinates::new(Column::new(5), Row::new(4)),
            Coordinates::new(Column::new(6), Row::new(4)), //
            Coordinates::new(Column::new(6), Row::new(3)), //
            Coordinates::new(Column::new(6), Row::new(2)),
            Coordinates::new(Column::new(5), Row::new(2)),
        ];

        assert_eq!(neighbours.len(), expected_neighbours.len());

        for expected in expected_neighbours {
            assert!(neighbours.contains(&expected))
        }
    }

    #[test]
    fn given_new_grid_with_isolated_cells_when_next_they_die() {
        let mut grid = Grid::new(Column::new(20), Row::new(20));

        let coordinates_1 = (Column::new(2), Row::new(3));
        let coordinates_2 = (Column::new(5), Row::new(12));
        grid.populate(coordinates_1.0, coordinates_1.1).unwrap();
        grid.populate(coordinates_2.0, coordinates_2.1).unwrap();

        grid.next();

        let cell_1 = grid
            .cell(coordinates_1.0, coordinates_1.1)
            .expect("some cell");
        let cell_2 = grid
            .cell(coordinates_2.0, coordinates_2.1)
            .expect("some cell");

        assert!(cell_1.is_empty());
        assert!(cell_2.is_empty());
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

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
