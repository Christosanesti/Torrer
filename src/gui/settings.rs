// Settings panel for GUI - Story 3.7 and 3.8 implementation
use gtk4::prelude::*;
use gtk4::{
    Box, Label, Switch, ComboBoxText, Button, ListBox, ListBoxRow, Entry, 
    Orientation, Separator, ProgressBar, ScrolledWindow
};
use std::sync::Arc;
use std::sync::Mutex;
use crate::error::TorrerResult;
use crate::config::{ConfigManager, Configuration};
use crate::bridge::{BridgeManager, Bridge};
use crate::bridge::collector::BridgeCollector;

/// Settings panel widget
pub struct SettingsPanel {
    container: Box,
    country_combo: ComboBoxText,
    ipv6_switch: Switch,
    mac_switch: Switch,
    bridge_list: ListBox,
    bridge_entry: Entry,
    collect_progress: ProgressBar,
    config_manager: Arc<Mutex<ConfigManager>>,
    bridge_manager: Arc<Mutex<BridgeManager>>,
}

impl SettingsPanel {
    /// Create a new settings panel
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 10);
        container.set_margin_top(10);
        container.set_margin_bottom(10);
        container.set_margin_start(10);
        container.set_margin_end(10);

        // Initialize managers
        let config_manager = Arc::new(Mutex::new(
            ConfigManager::new().unwrap_or_else(|_| ConfigManager::default())
        ));
        let bridge_manager = Arc::new(Mutex::new(
            BridgeManager::new().unwrap_or_else(|_| BridgeManager::default())
        ));

        // Country selection
        let country_box = Box::new(Orientation::Horizontal, 10);
        let country_label = Label::new(Some("Exit Country:"));
        country_label.set_halign(gtk4::Align::Start);
        country_label.set_hexpand(false);
        
        let country_combo = ComboBoxText::new();
        country_combo.append_text("Any");
        // Add common countries
        for country in &["US", "CA", "GB", "DE", "FR", "IT", "ES", "NL", "SE", "NO", "FI", "DK", "PL", "CZ", "AU", "NZ", "JP", "KR", "BR", "MX"] {
            country_combo.append_text(country);
        }
        country_combo.set_active(Some(0));
        
        country_box.append(&country_label);
        country_box.append(&country_combo);
        container.append(&country_box);

        // Load current country from config
        let config_mgr = config_manager.clone();
        let country_combo_clone = country_combo.clone();
        if let Ok(manager) = config_mgr.lock() {
            if let Ok(config) = manager.load() {
                if let Some(ref country) = config.country_code {
                    // Find and set the country in combo
                    // ComboBoxText items are indexed starting from 0
                    // Item 0 is "Any", items 1+ are countries
                    let countries = ["US", "CA", "GB", "DE", "FR", "IT", "ES", "NL", "SE", "NO", "FI", "DK", "PL", "CZ", "AU", "NZ", "JP", "KR", "BR", "MX"];
                    if let Some(pos) = countries.iter().position(|&c| c == country.as_str()) {
                        country_combo_clone.set_active(Some((pos + 1) as u32));
                    }
                }
            }
        }

        // Country selection handler
        let config_mgr_for_country = config_manager.clone();
        let countries_list = ["US", "CA", "GB", "DE", "FR", "IT", "ES", "NL", "SE", "NO", "FI", "DK", "PL", "CZ", "AU", "NZ", "JP", "KR", "BR", "MX"];
        country_combo.connect_changed(move |combo| {
            if let Some(active) = combo.active() {
                if active > 0 {
                    let idx = (active - 1) as usize;
                    if idx < countries_list.len() {
                        let country = countries_list[idx];
                        if let Ok(mut manager) = config_mgr_for_country.lock() {
                            if let Ok(mut config) = manager.load() {
                                config.country_code = Some(country.to_string());
                                let _ = manager.save(&config);
                            }
                        }
                    }
                } else {
                    // "Any" selected
                    if let Ok(mut manager) = config_mgr_for_country.lock() {
                        if let Ok(mut config) = manager.load() {
                            config.country_code = None;
                            let _ = manager.save(&config);
                        }
                    }
                }
            }
        });

        container.append(&Separator::new(Orientation::Horizontal));

        // IPv6 toggle
        let ipv6_box = Box::new(Orientation::Horizontal, 10);
        let ipv6_label = Label::new(Some("Enable IPv6:"));
        ipv6_label.set_halign(gtk4::Align::Start);
        ipv6_label.set_hexpand(false);
        
        let ipv6_switch = Switch::new();
        // Load current IPv6 setting
        let config_mgr = config_manager.clone();
        if let Ok(manager) = config_mgr.lock() {
            if let Ok(config) = manager.load() {
                ipv6_switch.set_active(config.ipv6_enabled);
            }
        }
        
        ipv6_box.append(&ipv6_label);
        ipv6_box.append(&ipv6_switch);
        container.append(&ipv6_box);

        // IPv6 switch handler
        let config_mgr_for_ipv6 = config_manager.clone();
        ipv6_switch.connect_state_set(move |switch, _| {
            if let Ok(mut manager) = config_mgr_for_ipv6.lock() {
                if let Ok(mut config) = manager.load() {
                    config.ipv6_enabled = switch.is_active();
                    let _ = manager.save(&config);
                }
            }
            glib::Propagation::Proceed
        });

        // MAC randomization toggle
        let mac_box = Box::new(Orientation::Horizontal, 10);
        let mac_label = Label::new(Some("MAC Randomization:"));
        mac_label.set_halign(gtk4::Align::Start);
        mac_label.set_hexpand(false);
        
        let mac_switch = Switch::new();
        mac_switch.set_active(false); // Default off
        
        mac_box.append(&mac_label);
        mac_box.append(&mac_switch);
        container.append(&mac_box);

        container.append(&Separator::new(Orientation::Horizontal));

        // Bridge management section
        let bridge_label = Label::new(Some("Bridge Management"));
        bridge_label.set_halign(gtk4::Align::Start);
        container.append(&bridge_label);

        // Bridge list
        let bridge_scroll = ScrolledWindow::new();
        bridge_scroll.set_min_content_height(150);
        let bridge_list = ListBox::new();
        bridge_scroll.set_child(Some(&bridge_list));
        container.append(&bridge_scroll);

        // Load existing bridges
        let bridge_mgr = bridge_manager.clone();
        let bridge_list_clone = bridge_list.clone();
        if let Ok(manager) = bridge_mgr.lock() {
            if let Ok(bridges) = manager.list_bridges() {
                for bridge in bridges {
                    let row = ListBoxRow::new();
                    let row_box = Box::new(Orientation::Horizontal, 10);
                    let bridge_label = Label::new(Some(&format!("{}:{}", bridge.address, bridge.port)));
                    bridge_label.set_halign(gtk4::Align::Start);
                    bridge_label.set_hexpand(true);
                    
                    let remove_btn = Button::with_label("Remove");
                    let bridge_mgr_for_remove = bridge_manager.clone();
                    let bridge_list_for_remove = bridge_list_clone.clone();
                    let addr = bridge.address.clone();
                    let port = bridge.port;
                    remove_btn.connect_clicked(move |_| {
                        if let Ok(manager) = bridge_mgr_for_remove.lock() {
                            if manager.remove_bridge(&addr, port).is_ok() {
                                // Refresh bridge list
                                bridge_list_for_remove.remove_all();
                                if let Ok(bridges) = manager.list_bridges() {
                                    for bridge in bridges {
                                        let row = ListBoxRow::new();
                                        let row_box = Box::new(Orientation::Horizontal, 10);
                                        let bridge_label = Label::new(Some(&format!("{}:{}", bridge.address, bridge.port)));
                                        bridge_label.set_halign(gtk4::Align::Start);
                                        bridge_label.set_hexpand(true);
                                        row_box.append(&bridge_label);
                                        row.set_child(Some(&row_box));
                                        bridge_list_for_remove.append(&row);
                                    }
                                }
                            }
                        }
                    });
                    
                    row_box.append(&bridge_label);
                    row_box.append(&remove_btn);
                    row.set_child(Some(&row_box));
                    bridge_list_clone.append(&row);
                }
            }
        }

        // Add bridge section
        let add_bridge_box = Box::new(Orientation::Horizontal, 10);
        let bridge_entry = Entry::new();
        bridge_entry.set_placeholder_text(Some("IP:Port (e.g., 1.2.3.4:443)"));
        let add_btn = Button::with_label("Add Bridge");
        
        let bridge_mgr_for_add = bridge_manager.clone();
        let bridge_list_for_add = bridge_list.clone();
        let entry_for_add = bridge_entry.clone();
        add_btn.connect_clicked(move |_| {
            if let Some(text) = entry_for_add.text().as_str().split(':').collect::<Vec<_>>().get(0..2) {
                if let (Ok(addr), Ok(port)) = (text[0].parse::<String>(), text[1].parse::<u16>()) {
                    let bridge = Bridge::new(addr, port);
                    if let Ok(manager) = bridge_mgr_for_add.lock() {
                        if manager.add_bridge(bridge.clone()).is_ok() {
                            // Add to list
                            let row = ListBoxRow::new();
                            let row_box = Box::new(Orientation::Horizontal, 10);
                            let bridge_label = Label::new(Some(&format!("{}:{}", bridge.address, bridge.port)));
                            bridge_label.set_halign(gtk4::Align::Start);
                            bridge_label.set_hexpand(true);
                            
                            let remove_btn = Button::with_label("Remove");
                            let bridge_mgr_for_remove = bridge_mgr_for_add.clone();
                            let bridge_list_for_remove = bridge_list_for_add.clone();
                            let addr = bridge.address.clone();
                            let port = bridge.port;
                            remove_btn.connect_clicked(move |_| {
                                if let Ok(manager) = bridge_mgr_for_remove.lock() {
                                    if manager.remove_bridge(&addr, port).is_ok() {
                                        bridge_list_for_remove.remove_all();
                                        if let Ok(bridges) = manager.list_bridges() {
                                            for bridge in bridges {
                                                let row = ListBoxRow::new();
                                                let row_box = Box::new(Orientation::Horizontal, 10);
                                                let bridge_label = Label::new(Some(&format!("{}:{}", bridge.address, bridge.port)));
                                                bridge_label.set_halign(gtk4::Align::Start);
                                                bridge_label.set_hexpand(true);
                                                row_box.append(&bridge_label);
                                                row.set_child(Some(&row_box));
                                                bridge_list_for_remove.append(&row);
                                            }
                                        }
                                    }
                                }
                            });
                            
                            row_box.append(&bridge_label);
                            row_box.append(&remove_btn);
                            row.set_child(Some(&row_box));
                            bridge_list_for_add.append(&row);
                            entry_for_add.set_text("");
                        }
                    }
                }
            }
        });
        
        add_bridge_box.append(&bridge_entry);
        add_bridge_box.append(&add_btn);
        container.append(&add_bridge_box);

        // Bridge collection section
        let collect_box = Box::new(Orientation::Horizontal, 10);
        let collect_btn = Button::with_label("Collect Bridges");
        let collect_progress = ProgressBar::new();
        collect_progress.set_visible(false);
        
        let bridge_mgr_for_collect = bridge_manager.clone();
        let bridge_list_for_collect = bridge_list.clone();
        let progress_for_collect = collect_progress.clone();
        collect_btn.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            progress_for_collect.set_visible(true);
            progress_for_collect.set_fraction(0.0);
            
            let bridge_mgr = bridge_mgr_for_collect.clone();
            let bridge_list = bridge_list_for_collect.clone();
            let progress = progress_for_collect.clone();
            let btn_clone = btn.clone();
            
            glib::spawn_future_local(async move {
                match BridgeCollector::new() {
                    Ok(mut collector) => {
                        progress.set_fraction(0.3);
                        match collector.collect_and_cache().await {
                            Ok(_) => {
                                progress.set_fraction(1.0);
                                glib::idle_add_local(move || {
                                    progress.set_visible(false);
                                    btn_clone.set_sensitive(true);
                                    
                                    // Refresh bridge list
                                    bridge_list.remove_all();
                                    if let Ok(manager) = bridge_mgr.lock() {
                                        if let Ok(bridges) = manager.list_bridges() {
                                            for bridge in bridges {
                                                let row = ListBoxRow::new();
                                                let row_box = Box::new(Orientation::Horizontal, 10);
                                                let bridge_label = Label::new(Some(&format!("{}:{}", bridge.address, bridge.port)));
                                                bridge_label.set_halign(gtk4::Align::Start);
                                                bridge_label.set_hexpand(true);
                                                
                                                let remove_btn = Button::with_label("Remove");
                                                let bridge_mgr_for_remove = bridge_mgr.clone();
                                                let bridge_list_for_remove = bridge_list.clone();
                                                let addr = bridge.address.clone();
                                                let port = bridge.port;
                                                remove_btn.connect_clicked(move |_| {
                                                    if let Ok(manager) = bridge_mgr_for_remove.lock() {
                                                        if manager.remove_bridge(&addr, port).is_ok() {
                                                            bridge_list_for_remove.remove_all();
                                                            if let Ok(bridges) = manager.list_bridges() {
                                                                for bridge in bridges {
                                                                    let row = ListBoxRow::new();
                                                                    let row_box = Box::new(Orientation::Horizontal, 10);
                                                                    let bridge_label = Label::new(Some(&format!("{}:{}", bridge.address, bridge.port)));
                                                                    bridge_label.set_halign(gtk4::Align::Start);
                                                                    bridge_label.set_hexpand(true);
                                                                    row_box.append(&bridge_label);
                                                                    row.set_child(Some(&row_box));
                                                                    bridge_list_for_remove.append(&row);
                                                                }
                                                            }
                                                        }
                                                    }
                                                });
                                                
                                                row_box.append(&bridge_label);
                                                row_box.append(&remove_btn);
                                                row.set_child(Some(&row_box));
                                                bridge_list.append(&row);
                                            }
                                        }
                                    }
                                    false
                                });
                            }
                            Err(e) => {
                                glib::idle_add_local(move || {
                                    progress.set_visible(false);
                                    btn_clone.set_sensitive(true);
                                    // Show error (would need dialogs module)
                                    false
                                });
                            }
                        }
                    }
                    Err(_) => {
                        glib::idle_add_local(move || {
                            progress.set_visible(false);
                            btn_clone.set_sensitive(true);
                            false
                        });
                    }
                }
            });
        });
        
        collect_box.append(&collect_btn);
        collect_box.append(&collect_progress);
        container.append(&collect_box);

        Self {
            container,
            country_combo,
            ipv6_switch,
            mac_switch,
            bridge_list,
            bridge_entry,
            collect_progress,
            config_manager,
            bridge_manager,
        }
    }

    /// Get the widget
    pub fn widget(&self) -> &Box {
        &self.container
    }
}

