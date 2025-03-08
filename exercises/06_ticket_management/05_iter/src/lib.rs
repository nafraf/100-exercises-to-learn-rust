use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
//
// Hint: just like in the previous exercise, you want to delegate the iteration to
//   the `Vec<Ticket>` field in `TicketStore`. Look at the standard library documentation
//   for `Vec` to find the right type to return from `iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl TicketStore {
    pub fn iter(&self) -> impl Iterator<Item = &Ticket> {
        // iter(&self) takes a shared reference to self (&self), indicating that
        // it doesn't modify the TicketStore instance or take ownership of it.
        // Instead, it simply provides read-only access to the collection's elements.
        // This is a crucial design choice that allows clients to examine
        // tickets multiple times without invalidating the collection.

        // The return type `impl Iterator<Item = &Ticket>`` employs Rust's
        // "impl Trait" syntax, which offers two key benefits. First, it hides
        // the concrete iterator type (likely std::slice::Iter<'_, Ticket>),
        // creating a cleaner API by focusing on the iterator's behavior
        // rather than its specific implementation.
        // Second, it clearly communicates that the iterator yields shared
        // references to Ticket objects (&Ticket), not owned instances.

        // The method body simply delegates to self.tickets.iter().
        // This suggests that TicketStore internally uses a collection
        // (likely a Vec<Ticket>) to store its data, and the implementation
        // leverages the underlying collection's iterator capabilities.
        // This composition-based approach follows Rust's principle of building
        // complex behaviors from simple components.
        self.tickets.iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
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
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
