use model::game_mode::GameMode;
use std::sync::mpsc;
use std::sync::mpsc::{SendError, RecvError};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    ModeSelected(GameMode),
}

pub struct Message {
    pub evt: Event,
    pub msg: String
}
/*
pub struct Sender {
    pub shadowed_sender: mpsc::Sender<Message>,
}

impl Sender {
    pub fn send(&self, msg: Message) -> Result<(), SendError<Message>> {
        debug!("Message sent: {}", msg.msg);
        self.shadowed_sender.send(msg)
    }
}

pub struct Receiver {
    pub shadowed_receiver: mpsc::Receiver<Message>,
}

impl Receiver{
    pub fn recv(&self) -> Result<Message, RecvError> {
        let result = self.shadowed_receiver.recv();
        match result {
            Ok(msg) => debug!("Message received: {}", msg.msg),
            Err(e) => error!("Error receiving message: {}", &e.to_string())
        };
        result
    }
}
*/