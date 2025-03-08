use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   Look at the standard library documentation for `Vec` to find the right type
//   to return from `into_iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

// See https://doc.rust-lang.org/std/vec/struct.IntoIter.html
impl IntoIterator for TicketStore {
    //  declares that the iterator will yield Ticket objects
    type Item = Ticket;
    // specifies that the actual iterator type being returned is a vector
    // iterator specialized for Ticket elements. This suggests that TicketStore
    // internally uses a Vec<Ticket> to store its data.
    type IntoIter = std::vec::IntoIter<Self::Item>;


    // The into_iter method implementation delegates to the into_iter method of
    // the internal tickets field.
    // This pattern of delegation is common in Rust when implementing traits for
    // wrapper types. By calling self.tickets.into_iter(), ownership of the internal
    // vector is transferred to the caller, allowing them to iterate through all
    // tickets without additional overhead.
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
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

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
