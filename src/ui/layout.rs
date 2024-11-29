use crate::system::cpu;

use ratatui::{
    Terminal,
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Frame,
};
use ratatui::text::Text;

pub fn render_ui<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
    terminal.draw(|f| {
        render_layout(f);
    })?;
    Ok(())
}

fn render_layout(f: &mut Frame) {
    let size = f.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ].as_ref())
        .split(size);

    let cpu_usage = cpu::get_cpu_usage();
    let cpu_text = Text::raw(format!("CPU Usage: {:.1}%", cpu_usage));

    let cpu_block = Paragraph::new(cpu_text)
        .block(Block::default().title("CPU Usage").borders(Borders::ALL));

    f.render_widget(cpu_block, layout[0]);

    // TODO: MEMORY USAGE
    let memory_block = Block::default()
        .title("Memory Usage")
        .borders(Borders::ALL);
    f.render_widget(memory_block, layout[1]);

// TODO: EXTRA STUFF LIKE DISK AND NETWORK USAGE
    let other_block = Block::default()
        .title("Other Information")
        .borders(Borders::ALL);
    f.render_widget(other_block, layout[2]);
}
