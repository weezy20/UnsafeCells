/*
A cell is a container with interior mutability such that it differs from
Refcell by only giving out bitwise copies of the interior data structure.
 */

#[cfg(test)]
mod tests {
    use super::Cell;
    #[test]
    fn mutate_unsafe() {
        let x = Cell::new(6);
        x.set(7);
        assert_eq!(7, x.get());
    }

    #[test]
    fn some_test() {
        assert!(true);
    }
}
use std::cell::UnsafeCell;
pub struct Cell<T> {
    val: UnsafeCell<T>,
}
impl<T> Cell<T> {
    pub fn new(val: T) -> Self {
        Cell {
            val: UnsafeCell::new(val),
        }
    }
    pub fn set(&self, val: T) {
        // mutate the interior of the cell
        unsafe { *self.val.get() = val };
    }
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.val.get() }
    }
}

// this is a comment
