/// For a space that is populated:
///     Each cell with one or no neighbors dies, as if by solitude.
///     Each cell with four or more neighbors dies, as if by overpopulation.
///     Each cell with two or three neighbors survives.
/// For a space that is empty or unpopulated
///     Each cell with three neighbors becomes populated.

enum Cell {
    Empty,
    Populated
}

impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
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
    }
}
