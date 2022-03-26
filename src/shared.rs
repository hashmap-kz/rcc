#![allow(non_camel_case_types)]

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct shared_ptr<T> {
    p: Rc<RefCell<T>>,
}

impl<T> shared_ptr<T> {
    pub fn new(p: T) -> Self {
        shared_ptr { p: Rc::new(RefCell::new(p)) }
    }

    pub fn _cloneref(another: &shared_ptr<T>) -> Self {
        // let z = another.p.clone();
        return shared_ptr {
            p: Rc::clone(&another.p)
        };
    }

    pub fn _bormut(&self) -> RefMut<T> {
        return self.p.as_ref().borrow_mut();
    }

    pub fn _bor(&self) -> Ref<T> {
        return self.p.as_ref().borrow();
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct shared_vec<T> {
    buf: shared_ptr<Vec<shared_ptr<T>>>,
}

impl<T> shared_vec<T> {
    pub fn new() -> Self {
        shared_vec {
            buf: shared_ptr::new(Vec::new())
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.buf._bor().is_empty();
    }

    pub fn len(&self) -> usize {
        return self.buf._bor().len();
    }

    pub fn get(&self, at: usize) -> shared_ptr<T> {
        assert!(at < self.len());
        return shared_ptr::_cloneref(self.buf._bor().get(at).unwrap());
    }

    pub fn set(&mut self, at: usize, e: shared_ptr<T>) -> shared_ptr<T> {
        assert!(at < self.len());
        let old = self.get(at);
        self.buf._bormut()[at] = e;
        return old;
    }

    pub fn push_back(&mut self, elem: shared_ptr<T>) {
        self.buf._bormut().push(elem);
    }

    pub fn pop_back(&mut self) -> shared_ptr<T> {
        assert!(!self.is_empty());
        let last = self.len() - 1;
        return self.buf._bormut().remove(last);
    }

    pub fn _borvec(&self) -> Ref<Vec<shared_ptr<T>>> {
        return self.buf._bor();
    }
}
