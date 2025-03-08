use crate::manager::WindowManager;
use crate::window::Window;

use anyhow::Context;
use hyprland::data::Clients;
use hyprland::dispatch::WindowIdentifier;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::shared::{Address, HyprData};
use std::collections::BTreeSet;

pub struct Hyprland {}

impl WindowManager for Hyprland {
    async fn fetch_windows(&self) -> anyhow::Result<Vec<Window>> {
        let clients = Clients::get_async().await?;

        let fetched: Vec<Window> = clients.into_iter().map(|w| Window::from(w)).collect();

        Ok(fetched)
    }

    async fn open_windows(&self, wins: Vec<&Window>) -> anyhow::Result<bool> {
        for window in wins {
            let command = format!(
                "[workspace {} silent; float; size {}, {}; move {}, {}] {}",
                window.workspace,
                window.size.0,
                window.size.1,
                window.at.0,
                window.at.1,
                window.program.cmdline
            );
            println!("{}", command);
            Dispatch::call_async(DispatchType::Exec(&command)).await?;
        }
        Ok(true)
    }

    async fn toggle_float(&self, wins: Vec<&Window>) -> anyhow::Result<bool> {
        for window in wins {
            Dispatch::call_async(DispatchType::ToggleFloating(Some(
                WindowIdentifier::Address(Address::new(window.address.clone())),
            )))
            .await?;
        }
        Ok(true)
    }

    async fn close_windows(&self, wins: Vec<&Window>) -> anyhow::Result<bool> {
        for window in wins {
            Dispatch::call_async(DispatchType::CloseWindow(WindowIdentifier::Address(
                Address::new(window.address.clone()),
            )))
            .await?;
        }
        Ok(true)
    }
}
