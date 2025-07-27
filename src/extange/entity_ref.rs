use crate::production::ProdInstance;

#[derive(Debug)]
pub enum EntityRef<'a> {
    Owned(ProdInstance),
    Borrowed(&'a mut ProdInstance),
}

impl<'a> EntityRef<'a> {
    pub fn as_ref(&self) -> &ProdInstance {
        match self {
            EntityRef::Owned(inst) => inst,
            EntityRef::Borrowed(inst) => inst,
        }
    }

    pub fn as_mut(&mut self) -> &mut ProdInstance {
        match self {
            EntityRef::Owned(inst) => inst,
            EntityRef::Borrowed(inst) => *inst,
        }
    }

    pub fn into_owned(self) -> ProdInstance {
        match self {
            EntityRef::Owned(inst) => inst,
            EntityRef::Borrowed(inst) => inst.clone(), // requires Clone
        }
    }
}
