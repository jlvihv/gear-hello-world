#![no_std]
use gstd::{debug, msg, prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    SendHelloTo(ActorId),
    SendHelloReply,
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let input_message: InputMessages = msg::load().expect("Error in loading InputMessages");
    match input_message {
        InputMessages::SendHelloTo(account) => {
            msg::send(account, String::from("Hello"), 0).expect("Error in sending Hello message to account");
        },
        InputMessages::SendHelloReply => {
            msg::reply(String::from("Hello"), 0).expect("Error in sending reply");
        }
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_message: String = msg::load().expect("Cant load init message");
    debug!("Program was initialized with message {:?}", init_message);
}

gstd::metadata! {
    title: "Hello world contract",
    handle:
        input: InputMessages,
        output: String,
}
