pub mod shared {
    pub fn solve_system(
        button_a: (u64, u64),
        button_b: (u64, u64),
        prize: (u64, u64),
    ) -> Option<u64> {
        let x1 = button_a.0 as f64;
        let x2 = button_b.0 as f64;

        let y1 = button_a.1 as f64;
        let y2 = button_b.1 as f64;

        let p1 = prize.0 as f64;
        let p2 = prize.1 as f64;

        let b = ((p2 * x1) - (y1 * p1)) / ((x1 * y2) - (x2 * y1));
        let a = (p1 - (x2 * b)) / x1;

        if a.fract() != 0.0 || b.fract() != 0.0 {
            return None;
        }

        Some((3 * a as u64) + b as u64)
    }
}
