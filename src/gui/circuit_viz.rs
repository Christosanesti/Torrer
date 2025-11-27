// Circuit visualization component
use gtk4::prelude::*;
use gtk4::{Box, Label, DrawingArea, Orientation, ScrolledWindow};
use crate::tor::circuit::CircuitInfo;
use std::sync::Arc;
use std::sync::Mutex;

/// Circuit visualization widget
pub struct CircuitVisualization {
    container: Box,
    drawing_area: DrawingArea,
    info_label: Label,
    circuits: Arc<Mutex<Vec<CircuitInfo>>>,
}

impl CircuitVisualization {
    /// Create a new circuit visualization
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 10);
        container.set_margin_top(10);
        container.set_margin_bottom(10);
        container.set_margin_start(10);
        container.set_margin_end(10);

        // Info label
        let info_label = Label::new(None);
        info_label.set_markup("<b>Active Circuits:</b>");
        container.append(&info_label);

        // Drawing area for circuit visualization
        let drawing_area = DrawingArea::new();
        drawing_area.set_content_width(500);
        drawing_area.set_content_height(300);
        
        let circuits = Arc::new(Mutex::new(Vec::new()));
        let circuits_clone = circuits.clone();
        
        drawing_area.set_draw_func(move |_, cr, width, height| {
            let circuits = circuits_clone.lock().unwrap();
            
            // Clear background
            cr.set_source_rgb(0.1, 0.1, 0.1);
            cr.paint().unwrap();

            if circuits.is_empty() {
                // Show placeholder
                cr.set_source_rgb(0.6, 0.6, 0.6);
                cr.select_font_face("Sans", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                cr.set_font_size(14.0);
                cr.move_to(width as f64 / 2.0 - 80.0, height as f64 / 2.0);
                cr.show_text("No active circuits").unwrap();
            } else {
                // Draw circuits
                let center_x = width as f64 / 2.0;
                let center_y = height as f64 / 2.0;
                let radius = (width.min(height) as f64 / 2.0) - 50.0;

                for (i, circuit) in circuits.iter().enumerate() {
                    let angle = (i as f64 * 2.0 * std::f64::consts::PI) / circuits.len() as f64;
                    let x = center_x + radius * angle.cos();
                    let y = center_y + radius * angle.sin();

                    // Draw node
                    cr.set_source_rgb(0.2, 0.6, 0.9);
                    cr.arc(x, y, 15.0, 0.0, 2.0 * std::f64::consts::PI);
                    cr.fill().unwrap();

                    // Draw connection to center
                    cr.set_source_rgb(0.4, 0.4, 0.4);
                    cr.set_line_width(2.0);
                    cr.move_to(center_x, center_y);
                    cr.line_to(x, y);
                    cr.stroke().unwrap();

                    // Draw circuit ID
                    cr.set_source_rgb(0.9, 0.9, 0.9);
                    cr.select_font_face("Sans", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                    cr.set_font_size(10.0);
                    cr.move_to(x + 20.0, y);
                    cr.show_text(&format!("Circuit {}", circuit.id)).unwrap();
                }

                // Draw center node (exit)
                cr.set_source_rgb(0.9, 0.3, 0.3);
                cr.arc(center_x, center_y, 20.0, 0.0, 2.0 * std::f64::consts::PI);
                cr.fill().unwrap();
            }
        });

        let scroll = ScrolledWindow::new();
        scroll.set_child(Some(&drawing_area));
        container.append(&scroll);

        Self {
            container,
            drawing_area,
            info_label,
            circuits,
        }
    }

    /// Update circuits
    pub fn update_circuits(&self, circuits: Vec<CircuitInfo>) {
        {
            let mut stored = self.circuits.lock().unwrap();
            *stored = circuits;
        }
        
        self.info_label.set_markup(&format!(
            "<b>Active Circuits:</b> {}",
            self.circuits.lock().unwrap().len()
        ));
        
        self.drawing_area.queue_draw();
    }

    /// Get the container widget
    pub fn widget(&self) -> &Box {
        &self.container
    }

    /// Get current circuits (for testing)
    pub fn get_circuits(&self) -> Vec<CircuitInfo> {
        self.circuits.lock().unwrap().clone()
    }
}

