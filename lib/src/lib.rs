use std::fmt::Debug;

pub mod point;

pub fn debug_grid<T: Debug>(vec: &Vec<Vec<T>>) {
    for row in vec {
        println!("{:?}", row);
    }
}
