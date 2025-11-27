// Dialog utilities for GUI - Story 3.7 and 3.8 implementation
use gtk4::prelude::*;
use gtk4::{AlertDialog, ApplicationWindow, AboutDialog, MessageDialog, ButtonsType, MessageType};
use crate::error::TorrerError;

/// Show an error dialog
pub fn show_error(parent: Option<&ApplicationWindow>, title: &str, message: &str, details: Option<&str>) {
    let dialog = AlertDialog::builder()
        .modal(true)
        .heading(title)
        .message(message)
        .detail(details)
        .build();

    if let Some(parent_window) = parent {
        dialog.set_transient_for(Some(parent_window));
    }
    dialog.show();
}

/// Show a TorrerError in a dialog
pub fn show_torrer_error(parent: Option<&ApplicationWindow>, error: &TorrerError) {
    let (title, message) = match error {
        TorrerError::Tor(msg) => ("Tor Error", msg),
        TorrerError::Iptables(msg) => ("iptables Error", msg),
        TorrerError::Config(msg) => ("Configuration Error", msg),
        TorrerError::Bridge(msg) => ("Bridge Error", msg),
        TorrerError::Dns(msg) => ("DNS Error", msg),
        TorrerError::Security(msg) => ("Security Error", msg),
        TorrerError::Io(msg) => ("IO Error", msg),
        TorrerError::Other(msg) => ("Error", msg),
    };
    
    show_error(parent, title, message, None);
}

/// Show an info dialog
pub fn show_info(parent: Option<&ApplicationWindow>, title: &str, message: &str) {
    let dialog = AlertDialog::builder()
        .modal(true)
        .heading(title)
        .message(message)
        .build();

    if let Some(parent_window) = parent {
        dialog.set_transient_for(Some(parent_window));
    }
    dialog.show();
}

/// Show a confirmation dialog
pub fn show_confirm(parent: Option<&ApplicationWindow>, title: &str, message: &str) -> bool {
    let dialog = MessageDialog::builder()
        .modal(true)
        .title(title)
        .text(message)
        .buttons(ButtonsType::YesNo)
        .message_type(MessageType::Question)
        .build();

    if let Some(parent_window) = parent {
        dialog.set_transient_for(Some(parent_window));
    }
    
    let response = dialog.run();
    dialog.close();
    
    response == gtk4::ResponseType::Yes
}

/// Show about dialog
pub fn show_about_dialog(parent: Option<&ApplicationWindow>) {
    let dialog = AboutDialog::builder()
        .program_name("Torrer")
        .version("1.0.0")
        .comments("Tor routing tool with GUI")
        .copyright("Copyright © 2024")
        .license_type(gtk4::License::Gpl30)
        .website("https://github.com/torrer/torrer")
        .build();

    if let Some(parent_window) = parent {
        dialog.set_transient_for(Some(parent_window));
    }
    
    dialog.show();
}

/// Show help dialog
pub fn show_help_dialog(parent: Option<&ApplicationWindow>) {
    let help_text = "Torrer GUI Help

Main Window:
- Start: Start Tor routing
- Stop: Stop Tor routing
- Restart: Restart Tor routing

Settings Tab:
- Exit Country: Select country for exit node
- IPv6: Enable/disable IPv6
- MAC Randomization: Enable/disable MAC address randomization
- Bridge Management: Add, remove, or collect bridges

Statistics Tab:
- View connection statistics and metrics

Circuits Tab:
- Visualize Tor circuits

Preferences Tab:
- Configure GUI preferences

For more information, visit: https://github.com/torrer/torrer";

    let dialog = MessageDialog::builder()
        .modal(true)
        .title("Torrer Help")
        .text(help_text)
        .buttons(ButtonsType::Ok)
        .message_type(MessageType::Info)
        .build();

    if let Some(parent_window) = parent {
        dialog.set_transient_for(Some(parent_window));
    }
    
    dialog.show();
}



