// Rc enables single-threaded multiple immutable ownership
// de-allocates the interior only when strong_count -> 0
// can also have non-ownership references of type Weak<T>
// !Sync+!Send
// Also note we can't store the ref count in Rc itself because
// on calling clone, each clone will have it's own ref count so it becomes tricky
// to de-allocate the Rc
// also RcInner.val must be cloneable which is why we cannot use
// just an RcInner<T> inside Rc. We must use a pointer to the data
// Rc has a *const pointer inside it, which reflects the fact that
// users of Rc shouldn't be able to mutate the interior value
// however, we need to increment the ref_count, so we use Cell

use crate::cell::Cell;
struct RcInner<T> {
    val: T,
    ref_count: Cell<usize>,
}

pub struct Rc<T> {
    inner: *const RcInner<T>,
}

impl<T> Rc<T> {
    pub fn new(val: T) -> Self {
        // we use Box specifically for a heap allocation
        let inner = Box::new(RcInner {
            val,
            ref_count: Cell::new(1),
        });
        Self {
            inner: Box::into_raw(inner),
        }
        // Using into_raw is essential as if we return Rc { inner: &*Box<T> }, the
        // Box will be dropped at the end of this scope.
        // Instead, taking a raw pointer, Box is consumed, but the interior memory
        // isn't freed. into_raw() returns a *mut T
        // Also you can't just return a Rc { inner: &*Box::new(RcInner<T>) } because
        // again Box will have no owner, and this will be a dangling reference.
    }

    pub fn strong_count(&self) -> usize {
        unsafe { &*self.inner }.ref_count.get()
    }
}

// Clone returns the exact same struct Rc, which is nothing but the same *const RcInner
// pointer, which is created with Rc::new() but only after increasing the ref_count.
// All that is shared between different Rc owners, is the pointer.

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let ptr = unsafe { &*self.inner };
        ptr.ref_count.set(ptr.ref_count.get() + 1);
        Self { inner: self.inner }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // self.inner is safe to deref because we know it will only be deallocated
        // once ref_count is 0.
        &unsafe { &*self.inner }.val
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let ptr = unsafe { &*self.inner };
        match ptr.ref_count.get() {
            1 => unsafe {
                drop(ptr);
                let _ = Box::from_raw(self.inner as *mut RcInner<T>);
                // we know no one has a shared ptr at this stage so it's fine
                // to cast it into a *mut pointer.
            },
            n => {
                ptr.ref_count.set(n - 1);
            }
        }
    }
}
