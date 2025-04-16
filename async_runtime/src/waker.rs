use std::task::{RawWaker, RawWakerVTable};

static VTABLE: RawWakerVTable = RawWakerVTable::new(my_clone, my_wake, my_wake_by_ref, my_drop);

unsafe fn my_clone(raw_waker: *const ()) -> RawWaker {
    RawWaker::new(raw_waker, &VTABLE)
}

// Convert the raw pointer back to a box and drops it.
unsafe fn my_wake(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

// TODO:
// Use AtomicBool that set to true in this
// Then executor check the AtomicBool before poll the feature

unsafe fn my_wake_by_ref(_raw_waker: *const ()) {}

// Convert bob back to a raw pointer and drop it.
unsafe fn my_drop(raw_waker: *const ()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

pub fn create_raw_waker() -> RawWaker {
    let data = Box::into_raw(Box::new(42u32)); // any data
    RawWaker::new(data as *const (), &VTABLE)
}
