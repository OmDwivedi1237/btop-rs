use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::system::cpu;
use crate::ui::widgets::cpu_graph::CpuGraph;

use ratatui::{
    Terminal,
    backend::Backend,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    Frame,
};

pub struct AppLayout {
    cpu_history: Arc<Mutex<VecDeque<f32>>>,
    cpu_graph: CpuGraph,
}

impl AppLayout {
    pub fn new() -> Self {
        let cpu_history = Arc::new(Mutex::new(VecDeque::with_capacity(100)));
        Self {
            cpu_history,
            cpu_graph: CpuGraph::new("CPU Usage".to_string()),
        }
    }

    pub fn update(&self) {
    let cpu_usage: f32 = cpu::get_cpu_usage();
        if let Ok(mut history) = self.cpu_history.lock() {
            if history.len() >= 100 {
                history.pop_front();
            }
            history.push_back(cpu_usage);
        }
    }
}
    
pub fn render_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app_layout: &AppLayout,
) -> Result<(), std::io::Error> {
    terminal.draw(|f| {
        render_layout(f, app_layout);
    })?;
    Ok(())
}

fn render_layout(f: &mut Frame, app_layout: &AppLayout) {
    let size = f.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ].as_ref())
        .split(size);

    // CPU Section
    app_layout.cpu_graph.render(f, layout[0], &app_layout.cpu_history);
    // Memory section
    let memory_info = Block::default()
        .title("Memory Usage")
        .borders(Borders::ALL);
    f.render_widget(memory_info, layout[1]);

    // Other information section
    let other_info = Block::default()
        .title("Other Information")
        .borders(Borders::ALL);
    f.render_widget(other_info, layout[2]);
}
