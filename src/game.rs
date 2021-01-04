/// For a space that is populated:
///     Each cell with one or no neighbors dies, as if by solitude.
///     Each cell with four or more neighbors dies, as if by overpopulation.
///     Each cell with two or three neighbors survives.
/// For a space that is empty or unpopulated
///     Each cell with three neighbors becomes populated.

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
}
