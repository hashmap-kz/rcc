use std::cell::{Ref, RefMut};
use crate::shared_ptr;
use crate::sym::Sym;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ident {
    pub name: String,
    pub sym: Option<shared_ptr<Sym>>,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Ident {
            name,
            sym: None,
        }
    }

    #[inline]
    pub fn set_sym(&mut self, sym: shared_ptr<Sym>) {
        self.sym = Some(sym);
    }

    #[inline]
    pub fn get_mut_sym(&self) -> RefMut<Sym> {
        // TODO: check
        return self.sym.as_ref().unwrap()._bormut();
    }

    #[inline]
    pub fn get_shared_sym(&self) -> &shared_ptr<Sym> {
        // TODO: check
        return self.sym.as_ref().unwrap();
    }

    #[inline]
    pub fn has_sym(&self) -> bool {
        return self.sym.is_some();
    }
}
