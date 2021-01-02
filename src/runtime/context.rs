use std::cell::RefCell;

use crate::runtime::handle::Handle;

thread_local!{
    static CURRENT: RefCell<Option<Handle>>= RefCell::new(None);
}

pub fn current() -> Option<Handle> {
    CURRENT.with(|ctx| ctx.borrow().clone())
}