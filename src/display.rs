use crate::config::Config;
use num_complex::Complex;
use std::io::{stdout, Stdout};
use tui::{
    backend::TermionBackend,
    widgets::{BarChart, Block, Borders},
    Terminal,
};

pub struct Display<'a> {
    config: &'a Config,
    terminal: Terminal<TermionBackend<Stdout>>,
}

impl<'a> Display<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut terminal = Terminal::new(TermionBackend::new(stdout())).unwrap();

        terminal.clear().unwrap();

        Self { config, terminal }
    }

    pub fn update(&mut self, data: &[Complex<f32>]) {
        let bar_width = self.config.bar_width;

        self.terminal
            .draw(move |f| {
                let data_dist = data
                    .iter()
                    .map(|x| ("", (x.re * x.re + x.im * x.im).round() as u64))
                    .collect::<Vec<(&str, u64)>>();

                let bar_chart = BarChart::default()
                    .block(Block::default().title("mvis").borders(Borders::ALL))
                    .bar_width(bar_width)
                    .data(&data_dist);

                f.render_widget(bar_chart, f.size());
            })
            .unwrap();
    }
}
