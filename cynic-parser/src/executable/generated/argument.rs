use super::prelude::*;
use super::{
    ids::{ArgumentId, ValueId},
    value::Value,
    ExecutableId,
};
#[allow(unused_imports)]
use std::fmt::{self, Write};

pub struct ArgumentRecord {
    pub name: StringId,
    pub value: ValueId,
}

#[derive(Clone, Copy)]
pub struct Argument<'a>(pub(in super::super) ReadContext<'a, ArgumentId>);

impl<'a> Argument<'a> {
    pub fn name(&self) -> &'a str {
        let document = &self.0.document;
        document.lookup(document.lookup(self.0.id).name)
    }
    pub fn value(&self) -> Value<'a> {
        let document = self.0.document;
        document.read(document.lookup(self.0.id).value)
    }
}

impl Argument<'_> {
    pub fn id(&self) -> ArgumentId {
        self.0.id
    }
}

impl fmt::Debug for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Argument")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl ExecutableId for ArgumentId {
    type Reader<'a> = Argument<'a>;
    fn read(self, document: &ExecutableDocument) -> Self::Reader<'_> {
        Argument(ReadContext { id: self, document })
    }
}

impl<'a> IdReader<'a> for Argument<'a> {
    type Id = ArgumentId;
    fn new(id: Self::Id, document: &'a ExecutableDocument) -> Self {
        document.read(id)
    }
}
