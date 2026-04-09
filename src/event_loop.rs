use crate::{
    error::Error,
    models::{args::EventArgs, bin_heap::BinHeap, event::Event},
    registry::get_event_functions,
};
use std::sync::{Arc, Mutex, OnceLock};
use std::{collections::HashMap, sync::Condvar};

pub struct EventLoop {
    pub bin_heap: BinHeap,
    pub funcs: HashMap<
        String,
        Box<dyn Fn(&HashMap<String, EventArgs>) -> Result<(), Error> + Send + Sync>,
    >,
}

static INSTANCE: OnceLock<Arc<(Mutex<EventLoop>, Condvar)>> = OnceLock::new();

impl EventLoop {
    fn new() -> EventLoop {
        EventLoop {
            bin_heap: BinHeap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn instance() -> &'static Arc<(Mutex<EventLoop>, Condvar)> {
        INSTANCE.get_or_init(|| Arc::new((Mutex::new(EventLoop::new()), Condvar::new())))
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
        )> = get_event_functions();
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

    pub fn push_event(&mut self, event: Event) {
        self.bin_heap.insert(event);

        let pair = EventLoop::instance().clone();
        let (_, cvar) = &*pair;
        cvar.notify_one();
    }
}
