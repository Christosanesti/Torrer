// Statistics dashboard with charts and graphs
use gtk4::prelude::*;
use gtk4::{Box, Label, DrawingArea, Orientation};
use crate::core::monitoring::Statistics;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;

/// Statistics dashboard widget
pub struct StatisticsDashboard {
    container: Box,
    uptime_label: Label,
    bytes_sent_label: Label,
    bytes_received_label: Label,
    success_rate_label: Label,
    chart_area: DrawingArea,
    data_history: Arc<Mutex<VecDeque<(f64, f64, f64)>>>, // (time, sent, received)
}

impl StatisticsDashboard {
    /// Create a new statistics dashboard
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 10);
        container.set_margin_top(10);
        container.set_margin_bottom(10);
        container.set_margin_start(10);
        container.set_margin_end(10);

        // Statistics labels
        let stats_box = Box::new(Orientation::Vertical, 5);
        
        let uptime_label = Label::new(None);
        uptime_label.set_markup("<b>Uptime:</b> --");
        stats_box.append(&uptime_label);

        let bytes_sent_label = Label::new(None);
        bytes_sent_label.set_markup("<b>Bytes Sent:</b> 0");
        stats_box.append(&bytes_sent_label);

        let bytes_received_label = Label::new(None);
        bytes_received_label.set_markup("<b>Bytes Received:</b> 0");
        stats_box.append(&bytes_received_label);

        let success_rate_label = Label::new(None);
        success_rate_label.set_markup("<b>Success Rate:</b> 0%");
        stats_box.append(&success_rate_label);

        container.append(&stats_box);

        // Chart area
        let chart_area = DrawingArea::new();
        chart_area.set_content_width(400);
        chart_area.set_content_height(200);
        chart_area.set_draw_func(|_, cr, width, height| {
            // Simple line chart drawing
            cr.set_source_rgb(0.2, 0.2, 0.2);
            cr.paint().unwrap();

            // Draw axes
            cr.set_source_rgb(0.8, 0.8, 0.8);
            cr.set_line_width(1.0);
            cr.move_to(20.0, height as f64 - 20.0);
            cr.line_to(width as f64 - 20.0, height as f64 - 20.0);
            cr.move_to(20.0, 20.0);
            cr.line_to(20.0, height as f64 - 20.0);
            cr.stroke().unwrap();

            // Draw placeholder text
            cr.set_source_rgb(0.6, 0.6, 0.6);
            cr.select_font_face("Sans", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
            cr.set_font_size(12.0);
            cr.move_to(width as f64 / 2.0 - 50.0, height as f64 / 2.0);
            cr.show_text("Chart will display here").unwrap();
        });

        container.append(&chart_area);

        let data_history = Arc::new(Mutex::new(VecDeque::with_capacity(100)));

        Self {
            container,
            uptime_label,
            bytes_sent_label,
            bytes_received_label,
            success_rate_label,
            chart_area,
            data_history,
        }
    }

    /// Update statistics display
    pub fn update_stats(&self, stats: &Statistics) {
        // Update uptime
        if let Some(uptime) = stats.uptime {
            let hours = uptime.as_secs() / 3600;
            let minutes = (uptime.as_secs() % 3600) / 60;
            let seconds = uptime.as_secs() % 60;
            self.uptime_label.set_markup(&format!(
                "<b>Uptime:</b> {:02}:{:02}:{:02}",
                hours, minutes, seconds
            ));
        } else {
            self.uptime_label.set_markup("<b>Uptime:</b> --");
        }

        // Update bytes
        self.bytes_sent_label.set_markup(&format!(
            "<b>Bytes Sent:</b> {}",
            Self::format_bytes(stats.bytes_sent)
        ));
        self.bytes_received_label.set_markup(&format!(
            "<b>Bytes Received:</b> {}",
            Self::format_bytes(stats.bytes_received)
        ));

        // Update success rate
        self.success_rate_label.set_markup(&format!(
            "<b>Success Rate:</b> {:.1}%",
            stats.success_rate
        ));

        // Update chart data
        let time = stats.uptime.map(|d| d.as_secs() as f64).unwrap_or(0.0);
        let mut history = self.data_history.lock().unwrap();
        history.push_back((time, stats.bytes_sent as f64, stats.bytes_received as f64));
        if history.len() > 100 {
            history.pop_front();
        }
        drop(history);

        // Redraw chart
        self.chart_area.queue_draw();
    }

    /// Format bytes to human-readable format
    fn format_bytes(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Get the container widget
    pub fn widget(&self) -> &Box {
        &self.container
    }
}





