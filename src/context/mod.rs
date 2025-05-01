use std::any::Any;
use std::sync::Arc;

use tokio::sync::Mutex;

pub struct Store {
    contexts: Vec<Box<dyn ContextProvider<Context = dyn Any>>>,
    inner: std::collections::HashMap<String, Arc<Mutex<dyn Any>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            contexts: Vec::new(),
            inner: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, ctx: impl ContextProvider) {}
}

pub trait ContextProvider {
    type Context;
}
