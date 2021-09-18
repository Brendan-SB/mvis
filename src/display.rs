use crate::config::Config;
use num_complex::Complex;
use std::io::{stdout, Stdout};
use tui::{
    backend::CrosstermBackend,
    widgets::{BarChart, Block, Borders},
    Terminal,
};

pub struct Display<'a> {
    config: &'a Config,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Display<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

        terminal.clear().unwrap();

        Self { config, terminal }
    }

    pub fn update(&mut self, data: &[Complex<f32>]) {
        self.terminal
            .draw(move |f| {
                let data_dist = data
                    .iter()
                    .map(|x| ("", (x.re * x.re + x.im * x.im).round() as u64))
                    .collect::<Vec<(&str, u64)>>();

                let bar_chart = BarChart::default()
                    .block(Block::default().title("mvis").borders(Borders::ALL))
                    .bar_width(3)
                    .data(&data_dist);

                f.render_widget(bar_chart, f.size());
            })
            .unwrap();
    }
}
