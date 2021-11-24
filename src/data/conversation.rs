use druid::{
    im::{vector, Vector},
    Data, Lens,
};

use super::contact::Contact;

#[derive(Clone, Data, Lens, Debug)]
pub struct Conversation {
    pub contact: Contact,
    messages: Vector<Msg>,
}

impl Conversation {
    pub fn new(contact: Contact) -> Self {
        Self {
            contact,
            messages: vector![],
        }
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct Msg {
    source_pk: String,
    content: String,
}

impl Msg {
    pub fn new(source_pk: &str, content: &str) -> Self {
        Self {
            source_pk: source_pk.into(),
            content: content.into(),
        }
    }
}
