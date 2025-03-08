use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Implement the `IntoIterator` trait for `&TicketStore` so that the test compiles and passes.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}


// The `impl<'a> IntoIterator for &'a TicketStore` declaration introduces a
// lifetime parameter 'a that connects the lifetime of the TicketStore reference
// to the lifetimes of the references yielded by the iterator.
// This ensures that the borrowed tickets cannot outlive the collection that
// contains them, preventing potential memory safety issues.
impl<'a>  IntoIterator for &'a TicketStore {
    // The `type Item = &'a Ticket` associated type declaration specifies that
    // the iterator will produce references to Ticket objects rather than owned
    // values. This allows for efficient, non-destructive iteration where the
    // original collection remains intact and available for future use.
    type Item = &'a Ticket;

    // The `type IntoIter = std::slice::Iter<'a, Ticket>` line reveals that the
    // implementation leverages Rust's built-in slice iterator, suggesting that
    // TicketStore internally stores tickets in a contiguous collection like a
    // vector or array. This is an efficient choice as slice iterators are
    // optimized for traversing elements stored in memory.
    type IntoIter = std::slice::Iter<'a, Ticket>;

    // the `into_iter`` method implementation simply delegates to the internal
    // collection's iter() method.
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
