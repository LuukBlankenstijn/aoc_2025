const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Self {
        Self { cells }
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn width(&self) -> usize {
        self.cells.first().map_or(0, |row| row.len())
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        self.cells.get(r)?.get(c)
    }

    pub fn replace(&mut self, r: usize, c: usize, new_value: T) -> Option<()> {
        if let Some(row) = self.cells.get_mut(r) {
            if let Some(cell) = row.get_mut(c) {
                *cell = new_value;
                return Some(());
            }
        }
        None
    }

    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize, &T)> {
        let mut out = Vec::new();
        for (dr, dc) in DIRS {
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0 && c >= 0 {
                let (r, c) = (r as usize, c as usize);
                if r < self.height() && c < self.width() {
                    if let Some(val) = self.get(r, c) {
                        out.push((r, c, val));
                    }
                }
            }
        }
        out
    }

    pub fn cells(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.cells.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
        })
    }
}
