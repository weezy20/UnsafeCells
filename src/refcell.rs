use crate::cell::Cell;
use std::cell::UnsafeCell;

pub struct RefCell<T> {
    val: UnsafeCell<T>,
    state: Cell<RefState>,
}

enum RefState {
    None,
    Shared(usize),
    Exclusive,
}

impl<T> RefCell<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: UnsafeCell::new(val),
            state: Cell::new(RefState::None),
        }
    }

    pub fn borrow(&'a self) -> Ref<'a, T> {}
    pub fn borrow_mut(&'a self) -> RefMut<'a, T> {}
}
/*
Ref<'_,T> and RefMut wrappers(smart pointers) around references to the RefCell
with some lifetime related to the scope in which the reference is valid
When the reference goes out of scope, our custom drop() is called which deals with
the decrement of shared/exclusive RefState

A Ref<'_, T> is supposed to be read like a & or &mut except, it's smart for the
reason because it implements additional semantics with traits like Drop, Deref and
DerefMut.

The reason we want to impl Deref is because a user expects, when calling borrow() on
RefCell<T>, to get a &T, not a weird Ref<'_, T>. If we impl deref however, the
compiler knows to call * on our type until it reaches the target &Self::Target
which we define in the trait impl as none other than type Target = T.
*/

pub struct Ref<'a, T> {
    reference: &'a RefCell<T>,
}
pub struct RefMut<'a, T> {
    reference: &'a RefCell<T>,
}
