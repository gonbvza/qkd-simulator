use std::collections::HashMap;

use crate::{
    core::{event_loop::EventLoopHandler, state::SimulationState},
    error::Error,
    events::{
        qkd::{handle_qkd_init, receive_pair},
        sifting::same_basis,
        state::{create_link, create_node},
    },
    models::{
        args::EventArgs,
        event::ScheduledEvent,
        event_types::{EventName, EventPayload},
    },
};

pub struct Registry {
    pub funcs: HashMap<
        EventName,
        Box<
            dyn Fn(EventPayload, i64, &mut SimulationState, &EventLoopHandler) -> Result<(), Error>
                + Send
                + Sync,
        >,
    >,
    state: SimulationState,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            funcs: HashMap::new(),
            state: SimulationState::new(),
        }
    }

    // Return all functions as (name, function pointer) tuples
    pub fn get_event_functions(
        &self,
    ) -> Vec<(
        EventName,
        Box<
            dyn Fn(EventPayload, i64, &mut SimulationState, &EventLoopHandler) -> Result<(), Error>
                + Send
                + Sync,
        >,
    )> {
        vec![
            (EventName::HandleQkdInit, Box::new(handle_qkd_init)),
            (EventName::ReceivePair, Box::new(receive_pair)),
            (EventName::CreateNode, Box::new(create_node)),
            (EventName::CreateLink, Box::new(create_link)),
            (EventName::SameBasis, Box::new(same_basis)),
        ]
    }

    pub fn push_func<F>(&mut self, name: EventName, func: F)
    where
        F: Fn(EventPayload, i64, &mut SimulationState, &EventLoopHandler) -> Result<(), Error>
            + Send
            + Sync
            + 'static,
    {
        self.funcs.insert(name, Box::new(func));
    }

    pub fn instantiate_functions(&mut self) {
        let funcs: Vec<(
            EventName,
            Box<
                dyn Fn(
                        EventPayload,
                        i64,
                        &mut SimulationState,
                        &EventLoopHandler,
                    ) -> Result<(), Error>
                    + Send
                    + Sync,
            >,
        )> = self.get_event_functions();
        for key in funcs {
            self.push_func(key.0, key.1);
        }
    }

    pub fn exec_event(
        &mut self,
        scheduled_event: ScheduledEvent,
        handle: &EventLoopHandler,
    ) -> Result<(), Error> {
        match self.funcs.get(&scheduled_event.event.name) {
            Some(function) => {
                if let Err(e) = function(
                    scheduled_event.event.payload,
                    scheduled_event.timestamp,
                    &mut self.state,
                    handle,
                ) {
                    eprintln!("Function execution failed: {}", e);
                    return Err(e);
                }

                Ok(())
            }
            None => Err(Error::NonExistantFunction(scheduled_event.event.name)),
        }
    }
}
