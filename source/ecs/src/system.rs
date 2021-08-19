use containers;
use utils::*;
use crate::{View, ViewTuple, Viewable};

pub trait System {
    fn prepare(&self) -> Box<dyn SystemExec>;
}

pub struct SystemExecData<V: ViewTuple, F: FnMut(&mut V)> {
    views: V,
    exec_func: F,
}

pub trait SystemExec {
    fn exec(&mut self);
}

impl<V: ViewTuple, F: FnMut(&mut V)> SystemExec for SystemExecData<V, F> {
    fn exec(&mut self) {
        (self.exec_func)(&mut self.views);
    }
}

pub struct SystemBuilder<ViewT = ()> {
    views: ViewT,
}

impl SystemBuilder<()> {
    pub fn new() -> Self {
        Self {
            views: (),
        }
    }
}

impl<ViewT> SystemBuilder<ViewT>
where ViewT: 'static
{
    pub fn add_view<V>(self, v: V)
        -> SystemBuilder<<ViewT as typelist::TypeListOp<V>>::OutAppend>
        where
            ViewT: typelist::TypeListOp<V>
    {
        SystemBuilder {
            views: utils::typelist::TypeListOp::append(self.views, v),
        }
    }

    pub fn finalize<F>(self, exec: F) -> Box<dyn SystemExec>
    where
        ViewT: ViewTuple,
        F: FnMut(&mut ViewT) + 'static
    {
        Box::new(SystemExecData {
            views: self.views,
            exec_func: exec,
        })
    }
}
