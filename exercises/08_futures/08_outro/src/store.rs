use crate::data::{Status, Ticket, TicketDraft};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::{collections::BTreeMap, sync::atomic::Ordering};

use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TicketId(pub u64);

pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: AtomicU64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: AtomicU64::new(0),
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter.load(Ordering::Relaxed));
        self.counter.fetch_add(1, Ordering::Release);
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };

        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);

        id
    }

    // The `get` method should return a handle to the ticket
    // which allows the caller to either read or modify the ticket.
    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}

impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}
