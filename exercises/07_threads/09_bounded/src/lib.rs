// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<TicketDraft>> {
        let (response_send, response_receive) = std::sync::mpsc::channel();
        let draft_for_return = draft.clone();
        let command = Command::Insert { draft: draft, response_channel: response_send };
        match self.sender.try_send(command) {
            Ok(()) => {
                match response_receive.recv() {
                    Ok(Ok(id)) => Ok(id),
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err(TrySendError::Disconnected(draft_for_return)),
                }
            },
            Err(TrySendError::Full(command)) => {
                if let Command::Insert { draft: draft, response_channel } = command {
                    Err(TrySendError::Full(draft))
                } else {
                unreachable!("Insert. Got something else.")
            }
        },
        Err(TrySendError::Disconnected(command)) => {
            if let Command::Insert { draft: draft, response_channel } = command {
                Err(TrySendError::Disconnected(draft))
            } else {
                unreachable!("Insert. Got something else.")
            }
        }
        }

    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<TicketId>> {
        let (response_send, response_receive) = std::sync::mpsc::channel();
        let id_for_return = id.clone();
        let command = Command::Get { id: id, response_channel: response_send };
        match self.sender.try_send(command) {
            Ok(()) => {
                match response_receive.recv() {
                    Ok(ticket_id) => ticket_id,
                    Ok(Err(e)) => Err(e),
                    Err(_) => Err(TrySendError::Disconnected(id_for_return)),
                }
            },
            Err(TrySendError::Full(command)) => {
                if let Command::Get { id: ticket_id, response_channel } = command {
                    Err(TrySendError::Full(id))
                } else {
                    unreachable!("Get. Got something else.")
                }
            },
            Err(TrySendError::Disconnected(command)) => {
                if let Command::Get { id: ticket_id, response_channel } = command {
                    Err(TrySendError::Disconnected(id))
                } else {
                    unreachable!("Get. Got something else.")
                }
            }
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender: sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<Result<TicketId, TrySendError<TicketDraft>>>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Result<Option<Ticket>, TrySendError<TicketId>>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(Ok(id));
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id).unwrap().clone();
                response_channel.send(Ok(Some(ticket)));
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
