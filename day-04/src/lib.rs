pub mod shared {
    pub fn in_grid(x: i32, y: i32, n: usize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < n && (y as usize) < n
    }
}
