#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use tauri::{Manager, State, Window};
use tokio::time::sleep;

#[tauri::command]
fn channel_select(id: u64, state: State<StateLock>) {
    println!("e");
    state.set_current_channel(id.into());
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.manage(StateLock::new(window.clone()));
            tokio::spawn(backend(window));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![channel_select])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod utils;
pub use utils::Id;

pub use state::{Channel, HarmoniumState, Message, StateLock, User};
mod state {
    use std::{
        collections::HashMap,
        sync::{Mutex, MutexGuard},
    };

    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use tauri::Window;

    use crate::Id;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: Id<User>,
        pub name: String,
        pub image_url: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Message {
        pub id: Id<Message>,
        pub channel_id: Id<Channel>,
        pub author_id: Id<User>,
        pub content: String,
    }

    impl Message {
        pub fn new(
            id: Id<Message>,
            content: String,
            channel_id: Id<Channel>,
            author_id: Id<User>,
        ) -> Self {
            Self {
                author_id,
                channel_id,
                content,
                id,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Channel {
        pub id: Id<Channel>,
        pub name: String,
        pub messages: Vec<Id<Message>>,
    }

    impl Channel {
        pub fn new(id: Id<Channel>, name: String) -> Self {
            let messages = vec![];
            Self { id, name, messages }
        }
    }

    #[derive(Debug, Default)]
    pub struct HarmoniumState {
        current_channel: Option<Id<Channel>>,
        users: HashMap<Id<User>, User>,
        channels: HashMap<Id<Channel>, Channel>,
        messages: HashMap<Id<Message>, Message>,
    }

    pub struct StateLock {
        state: Mutex<HarmoniumState>,
        win_handle: Window,
    }

    impl StateLock {
        pub fn new(win_handle: Window) -> Self {
            let state = Default::default();
            Self { win_handle, state }
        }

        pub fn lck(&self) -> MutexGuard<HarmoniumState> {
            self.state.lock().unwrap()
        }

        pub fn add_channel(&self, channel: Channel) {
            self.win_handle.emit("add_channel", &channel).unwrap();
            self.lck()
                .channels
                .insert(channel.id.clone(), channel.clone());
            println!("sent {channel:?}")
        }

        pub fn get_channel(&self, id: &Id<Channel>) -> Option<Channel> {
            self.lck().channels.get(id).cloned()
        }

        pub fn add_message(&self, message: Message) {
            self.lck()
                .channels
                .get_mut(&message.channel_id)
                .unwrap()
                .messages
                .push(message.id.clone());
            self.lck().messages.insert(message.id.clone(), message);
        }

        pub fn get_message(&self, id: &Id<Message>) -> Option<Message> {
            self.lck().messages.get(id).cloned()
        }

        pub fn set_current_channel(&self, id: Id<Channel>) {
            self.lck().current_channel = Some(id.clone());
            let channel = self.get_channel(&id).unwrap();
            let messages = (channel.messages.iter())
                .map(|id| self.get_message(id))
                .collect::<Vec<_>>();
            self.win_handle
                .emit(
                    "set_current_channel",
                    json!({"channel": channel, "messages": messages}),
                )
                .unwrap();
        }
    }
}

async fn backend(window: Window) {
    sleep(Duration::from_millis(500)).await;

    let lck = window.state::<StateLock>();
    lck.add_channel(Channel::new(87689376.into(), "a channel".into()));
    lck.add_channel(Channel::new(876893766.into(), "another channel".into()));
    lck.add_channel(Channel::new(8768937644.into(), "a third channel".into()));
    for i in 7896..7996 {
        lck.add_message(Message::new(
            i.into(),
            format!("hello from 'a channel' #{i}"),
            87689376.into(),
            876869376.into(),
        ))
    }
}
