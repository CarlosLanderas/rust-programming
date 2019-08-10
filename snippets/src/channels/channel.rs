
use std::sync::mpsc::{channel, sync_channel};
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::string::FromUtf8Error;

pub struct InMemoryData {
    payload: Vec<u8>,
}

impl InMemoryData {
    pub fn from(payload: &[u8]) -> InMemoryData
    {
        InMemoryData {
            payload : Vec::from(payload),
        }
    }
    pub fn len(&self) -> usize {
        self.payload.len()
    }

    pub fn payload_string(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.payload)
    }
}

#[test]
fn channel_test() {

    let (sender, receiver) = channel();

    let mut message = Arc::new(Mutex::new("This is a secret message".to_string()));
    let msg_ref = message.clone();

    thread::spawn(move || {

        thread::sleep(Duration::from_millis(100));

        let mut message = message.lock().unwrap();
        *message = String::from("Altered secret message");
        let data = InMemoryData::from(message.as_bytes());

        sender.send(data).unwrap();

    });

    let memory_message = receiver.recv().unwrap();

    assert_eq!(memory_message.payload_string().unwrap(), *msg_ref.lock().unwrap());


}




