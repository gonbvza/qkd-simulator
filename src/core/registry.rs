use std::collections::HashMap;

use crate::{
    error::Error,
    events::qkd::{handle_qkd_init, receive_pair},
    models::{args::EventArgs, event::Event},
};

pub struct Registry {
    pub funcs: HashMap<
        String,
        Box<dyn Fn(&HashMap<String, EventArgs>) -> Result<(), Error> + Send + Sync>,
    >,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            funcs: HashMap::new(),
        }
    }

    // Return all functions as (name, function pointer) tuples
    pub fn get_event_functions(
        &self,
    ) -> Vec<(
        &'static str,
        Box<dyn Fn(&HashMap<String, EventArgs>) -> Result<(), Error> + Send + Sync>,
    )> {
        vec![
            ("handle_qkd_init", Box::new(handle_qkd_init)),
            ("receive_pair", Box::new(receive_pair)),
        ]
    }

    pub fn push_func<F>(&mut self, name: String, func: F)
    where
        F: Fn(&HashMap<String, EventArgs>) -> Result<(), Error> + Send + Sync + 'static,
    {
        self.funcs.insert(name, Box::new(func));
    }

    pub fn instantiate_functions(&mut self) {
        let funcs: Vec<(
            &'static str,
            Box<dyn Fn(&HashMap<String, EventArgs>) -> Result<(), Error> + Send + Sync>,
        )> = self.get_event_functions();
        for key in funcs {
            self.push_func(key.0.to_string(), key.1);
        }
    }

    pub fn exec_event(&mut self, event: Event) -> Result<(), Error> {
        match self.funcs.get(&event.function) {
            Some(function) => function(&event.args),
            None => Err(Error::NonExistantFunction(event.function)),
        }
    }
}
