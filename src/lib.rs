#![allow(non_snake_case, unused)]
pub mod cell;
pub mod new;
pub mod rc;
pub mod refcell;

#[cfg(test)]
mod lib_tests {
    use super::refcell::RefCell;
    #[test]
    fn test_refcell() {
        // test creation, mutation, and dropping
        let cell = RefCell::new(42);
        let cell_string = RefCell::new(String::from("hello"));
        let cell_borrow = cell.borrow();
        assert_eq!(42, *cell_borrow.unwrap());
        assert_eq!("hello".to_string(), *cell_string.borrow().unwrap());
    }

    #[test]
    fn test_rc_creation() {
        use crate::rc::Rc;
        // create several clones of one rc and check ref_count
        let rc1 = Rc::new(String::from("Jamaica"));
        let rc2 = Rc::clone(&rc1);
        let rc3 = Rc::clone(&rc1);
        let rc4 = Rc::clone(&rc2);

        assert_eq!(4, Rc::strong_count(&rc1));
        assert_eq!("Jamaica".to_string(), *rc1);
        assert_eq!("Jamaica".to_string(), *rc2);
        assert_eq!("Jamaica".to_string(), *rc3);
        assert_eq!("Jamaica".to_string(), *rc4);
    }
}
