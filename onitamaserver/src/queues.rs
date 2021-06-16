// use std::collections::HashMap;
// use std::sync::Mutex;
// use tokio::sync::broadcast;
// use uuid::Uuid;
//
// type Sender = broadcast::Sender<Vec<u8>>;
// type Receiver = broadcast::Receiver<Vec<u8>>;
// pub struct ServerQueueMap {
//     queues: Mutex<HashMap<Uuid, Sender>>,
// }
//
// impl ServerQueueMap {
//     pub fn new() -> ServerQueueMap {
//         let map = HashMap::new();
//         ServerQueueMap {
//             queues: Mutex::new(map),
//         }
//     }
//     pub fn new_queue(&mut self) -> (Uuid, Sender) {
//         let key = Uuid::new_v4();
//         let mut queues = self.queues.lock().unwrap();
//         let (sender, _receiver) = broadcast::channel(2);
//         queues.insert(key, sender.clone());
//         (key, sender)
//     }
//     pub fn join_queue(&self, key: Uuid) -> Result<Sender,()> {
//         let queues = self.queues.lock().unwrap();
//         match queues.get(&key) {
//             Some(sender) => Ok(sender.clone()),
//             None => Err(()),
//         }
//     }
// }
