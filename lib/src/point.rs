use std::{
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign},
};

pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }

    pub fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn signum(self, other: Self) -> Self {
        Point::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }

    pub fn x_idx(&self) -> usize {
        self.x as usize
    }

    pub fn y_idx(&self) -> usize {
        self.y as usize
    }

    pub fn is_on_grid(&self, n: usize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x_idx() < n && self.y_idx() < n
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl AddAssign<(i32, i32)> for Point {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Index<Point> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        &self[point.y_idx()][point.x_idx()]
    }
}

impl<T> IndexMut<Point> for Vec<Vec<T>> {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self[point.y_idx()][point.x_idx()]
    }
}
