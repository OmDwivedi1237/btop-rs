use std::collections::VecDeque;
use crate::system::cpu;
use std::sync::{Arc, Mutex};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Block, Borders, Chart, Dataset, Axis},
};

pub struct CpuGraph {
    title: String,
}

impl CpuGraph {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        cpu_history: &Arc<Mutex<VecDeque<f32>>>
    ) {

        let cpu_usage: f32 = cpu::get_cpu_usage();

        // Safely get the lock or return early if poisoned
        let cpu_data = match cpu_history.lock() {
            Ok(guard) => guard,
            Err(_) => return, // Early return if mutex is poisoned
        };

        // Create dataset from CPU history
        let dataset: Vec<(f64, f64)> = cpu_data
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as f64, f64::from(value)))
            .collect();

        let datasets = vec![Dataset::default()
            .data(&dataset)
            .name("CPU Usage")
            .marker(symbols::Marker::Braille)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Cyan))];

        let chart = Chart::new(datasets)
            .block(
                Block::default()
                    .title(self.title.clone())
                    .borders(Borders::ALL)
            )
            .x_axis(
                Axis::default()
                    .title(Span::styled("Time", Style::default().fg(Color::Gray)))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, (cpu_data.len() as f64).max(1.0)])
            )
            .y_axis(
                Axis::default()
                .title(Span::styled(
                    format!("CPU Usage: {:.1}%", cpu_usage),
                    Style::default().fg(Color::Gray)
                ))
                                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
            );

        frame.render_widget(chart, area);
    }
}
