use super::host::{Invocation, InvocationResponse};
use crossbeam::{Receiver, Sender};
use std::collections::HashMap;

pub(crate) type InvokerPair = (Sender<Invocation>, Receiver<InvocationResponse>);

#[derive(Default)]
pub struct Router {
    routes: HashMap<String, InvokerPair>,
}

impl Router {
    pub fn add_route(
        &mut self,
        id: String,
        inv_s: Sender<Invocation>,
        resp_r: Receiver<InvocationResponse>,
    ) {
        self.routes.insert(id, (inv_s, resp_r));
    }

    pub fn get_pair(&self, id: &str) -> Option<InvokerPair> {
        match self.routes.get(id) {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }
}
