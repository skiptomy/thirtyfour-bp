use std::{collections::HashMap, any::{TypeId, Any}, sync::Arc};

use cucumber::World;
use tokio::io;

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


#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct E2eWorld {
    pub aws_account_id: Option<String>,
    pub context: Arc<Context>,
}

impl E2eWorld {
    async fn new() -> io::Result<Self> {
        Ok(Self {
            aws_account_id: Some("123456789012".to_string()),
            context: Default::default(),
        })
    }
}
