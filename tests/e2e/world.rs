use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc, convert::Infallible,
};
use async_trait::async_trait;

use cucumber::{World, WorldInit};

#[derive(Debug, Default)]
pub struct Context {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|x| x.downcast_ref::<T>())
    }

    pub fn insert<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }
}

#[derive(Debug, WorldInit, Clone)]
pub struct E2eWorld {
    pub aws_account_id: Option<String>,
    pub context: Arc<Context>,
}

#[async_trait(?Send)]
impl World for E2eWorld {
    type Error = Infallible;
    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            aws_account_id: Some("123456789012".to_string()),
            context: Default::default(),
        })
    }
}
