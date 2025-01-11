use std::collections::VecDeque;
use crate::system::memory;
use std::sync::{Arc, Mutex};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Block, Borders, Chart, Dataset, Axis},
};

pub struct MemGraph {
    title: String,
}

impl MemGraph {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(
        &self,
        frame: &mut Frame,  
        area: Rect,
        mem_history: &Arc<Mutex<VecDeque<f32>>>
    ) {

        let mem_usage: f32 = memory::get_memory_usage();

        let mem_data = match mem_history.lock() {
            Ok(guard) => guard,
            Err(_) => return, // Early return if mutex is poisoned
        };

        let dataset: Vec<(f64, f64)> = mem_data
            .iter()
            .enumerate()
            .map(|(i, &value)| (i as f64, f64::from(value)))
            .collect();

        let datasets = vec![Dataset::default()
            .data(&dataset)
            .name("Memory Usage")
            .marker(symbols::Marker::Braille)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(Color::Magenta))];

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
                    .bounds([0.0, (mem_data.len() as f64).max(1.0)])
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled(
                        format!("Memory Usage: {:.1}%", mem_usage),
                        Style::default().fg(Color::Gray)
                    ))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
            );

        frame.render_widget(chart, area);
    }
}
