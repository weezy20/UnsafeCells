#![allow(non_snake_case)]
pub mod cell;
pub mod new;
pub mod refcell;

#[cfg(test)]
mod tests {
    use super::refcell::RefCell;
    #[test]
    fn test_refcell() {
        // test creation, mutation, and dropping
        let cell = RefCell::new(42);
        let cell_string = RefCell::new(String::from("hello"));
        let cell_borrow = cell.borrow();
        assert_eq!(42, *cell_borrow.unwrap());
        assert_eq!("hello".to_string(), *cell_string.unwrap());
    }
}
