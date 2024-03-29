use std::rc::{Weak, Rc};

use super::{Container, Object, TryAsRef};

#[derive(Clone, Debug)]
pub struct Pointer {
    pub(crate) container: Option<Weak<Container>>,
    pub(crate) index: Option<usize>,
}

impl Default for Pointer {
    fn default() -> Self { Self::NULL }
}

impl Pointer {
    pub(crate) const NULL: Pointer = Pointer { container: None, index: None };

    pub(crate) fn new(container: &Rc<Container>, index: usize) -> Self {
        Pointer {
            container: Some(Rc::downgrade(container)),
            index: Some(index),
        }
    }

    pub(crate) fn to_start_of_container(container: &Rc<Container>) -> Self {
        Self::new(container, 0)
    }

    pub(crate) fn to(object: &Object) -> Self {
        let container: Rc<Container> = object.parent().unwrap().try_as_ref().cloned().unwrap();
        let index = container.index_of(object);
        Self { 
            container: Some(Rc::downgrade(&container)), 
            index,
        }
    }

    pub(crate) fn container(&self) -> Option<Rc<Container>> {
        self.container
            .as_ref()
            .and_then(Weak::upgrade)
            .map(|c| c)
    }

    pub(crate) fn resolve(&self) -> Option<Object> {
        let container = self.container.as_ref()?.upgrade()?;
        match self.index {
            None => Some(Object::Container(container.clone())),
            Some(index) => container.content.get(index).cloned(),
        }
    }

    pub(crate) fn is_null(&self) -> bool {
        self.container.is_none() && self.index.is_none()
    }

    pub(crate) fn increment_index(&mut self) {
        self.index = self.index.map(|i| i + 1);
    }
}
