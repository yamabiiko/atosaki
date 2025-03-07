use crate::manager::WindowManager;
use crate::window::Window;

use anyhow::Context;
use hyprland::data::{Clients, Client};
use hyprland::shared::{HyprData, Address};
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::dispatch::WindowIdentifier;
use std::collections::HashMap;


pub struct Hyprland {
}

#[repr(u32)]
#[derive(Debug)]
enum EventType {
    Add = 0,
    Update = 1,
    Delete = 2,
}

impl EventType {
    fn from_u32(value: u32) -> Option<EventType> {
        match value {
            0 => Some(EventType::Add),
            1 => Some(EventType::Update),
            2 => Some(EventType::Delete),
            _ => None,
        }
    }
}

impl WindowManager for Hyprland {
    async fn update_session(&self) -> (HashMap<String, Window>, anyhow::Result<bool>) {
        let clients = Clients::get_async().await.unwrap();

        let new_data: HashMap<String, Window> = clients
            .into_iter()
            .map(|w| (w.address.to_string(), Window::from(w)))
            .collect();

        (new_data, Ok(true))
    }
    async fn open_session(&self, w_data: &HashMap<String, Window>) -> anyhow::Result<bool> {
        let clients = Clients::get_async().await.unwrap();
        let new_data: HashMap<String, Window> = clients
            .into_iter()
            .map(|w| (w.address.to_string(), Window::from(w)))
            .collect();


        for (_, window) in w_data {
            if let Some(_) = new_data.get(&window.address) {
                continue;
            }
            let command = format!(
                " [workspace {} silent; float; size {}, {}; move {}, {}; unset float] {}",
                                    window.workspace + 1,
                                    window.size.0, window.size.1,
                                    window.at.0, window.at.1,
                                    window.program.cmdline
                                );
            println!("{}", command);
            Dispatch::call_async(DispatchType::Exec(&command)).await?;
        }
        Ok(true)
    }

    async fn close_session(&self, w_data: &HashMap<String, Window>) -> anyhow::Result<bool> {
        for (_, window) in w_data {
            Dispatch::call_async(DispatchType::CloseWindow(WindowIdentifier::Address(Address::new(window.address.clone())))).await?;
        }
        Ok(true)
    }
}
