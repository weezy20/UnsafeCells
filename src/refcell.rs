use crate::cell::Cell;
use std::cell::UnsafeCell;
// RefCell is a RAII guard pattern
// RAII stands for resource acquisiton is initiation
// which means that objects are tied to resources
// so their instantiation and going out of scope
// are tied to resource management. A RAII Guard
// on the other hand is a more flexible design pattern
// which may relax these constraints to allow for situations like delaying
// resource acquisiton or allow usage after scope ends
// Ref/RefMut and MutexGuard are types of RAII Guard objects

pub struct RefCell<T> {
    val: UnsafeCell<T>,
    state: Cell<RefState>,
}
// We only need Copy on RefState because we'll be wrapping them
// in a Cell for interior mutability but since Clone
// is a super-trait for Copy, we derive both of them.
#[derive(Copy, Clone)]
enum RefState {
    None,
    Shared(usize),
    Exclusive,
}

impl<T> RefCell<T> {
    /// Creates a new RefCell<T>
    ///
    /// # Example:
    ///
    /// ```
    ///
    /// let cell = RefCell::new(42);
    /// let cell_string = RefCell::new(String::from("hello"));
    /// let cell_borrow = cell.borrow();
    /// assert_eq!(42, *cell_borrow.unwrap());
    /// assert_eq!("hello".to_string(), *cell_string.borrow().unwrap());
    /// ```
    pub fn new(val: T) -> Self {
        Self {
            val: UnsafeCell::new(val),
            state: Cell::new(RefState::None),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::None => {
                self.state.set(RefState::Shared(1));
                Some(Ref { reference: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { reference: self })
            }
            RefState::Exclusive => None,
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::None => {
                self.state.set(RefState::Exclusive);
                Some(RefMut { reference: self })
            }
            RefState::Shared(_) => None,
            RefState::Exclusive => None,
        }
    }
}
/*
Ref<'_,T> and RefMut wrappers(smart pointers) around references to the RefCell
with some lifetime related to the scope in which the reference is valid
When the reference goes out of scope, our custom drop() is called which deals with
the decrement of shared/exclusive RefState.

Also note that the lifetime of a Ref is tied to the RefCell because we disallow any Refs
when the RefCell itself has gone out of scope.

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

impl<T> std::ops::Deref for Ref<'_, T> {
    // A Ref<> is created only when no exclusive references exist
    // which is checked at runtime rather than at compilation
    // so dereferencing the *mut T given by UnsafeCell.get()
    // and casting it into & is fine
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.reference.val.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        // On drop we must decrement the RefState(Shared) count
        match self.reference.state.get() {
            RefState::None | RefState::Exclusive => unreachable!(),
            RefState::Shared(1) => self.reference.state.set(RefState::None),
            RefState::Shared(n) => self.reference.state.set(RefState::Shared(n - 1)),
        }
    }
}
// RefMut must implement both Deref and DerefMut traits.
impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.reference.val.get() }
    }
}

// RefMut is only created when no shared references have been given out
// Once a RefMut is given out, we set the state to Exclusive disallowing
// any Refs to exist.

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.reference.val.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.reference.state.get() {
            RefState::Shared(_) | RefState::None => unreachable!(),
            RefState::Exclusive => self.reference.state.set(RefState::None),
        }
    }
}
