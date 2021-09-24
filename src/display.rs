use crate::{config::Config, consts::PROGRAM_NAME};
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

    fn calculate_offset(data_dist_len: f32, bar_width: f32, terminal_width: f32) -> usize {
        ((data_dist_len + 1_f32 + bar_width * data_dist_len) / terminal_width).round()
            as usize
    }

    fn group_bars(data: &[Complex<f32>], bar_width: f32, terminal_width: f32) -> Vec<(&str, u64)> {
        let mut data_dist_reformed = Vec::new();

        {
            let data_dist = data
                .iter()
                .map(|x| (x.re * x.re + x.im * x.im).round())
                .collect::<Vec<f32>>();

            let offset = Self::calculate_offset(data_dist.len() as f32, bar_width, terminal_width);

            for i in (0..data_dist.len() - offset).step_by(offset) {
                let mut sum = 0_f32;

                for j in i..=i + offset {
                    sum += data_dist[j];
                }

                data_dist_reformed.push(("", (sum / offset as f32).round() as u64));
            }
        }

        data_dist_reformed
    }

    pub fn update(&mut self, data: &[Complex<f32>]) {
        let bar_width = self.config.bar_width;
        let terminal_width = self.terminal.size().unwrap().width;

        self.terminal
            .draw(move |f| {
                let data_dist = Self::group_bars(data, bar_width as f32, terminal_width as f32);

                let bar_chart = BarChart::default()
                    .block(Block::default().title(PROGRAM_NAME).borders(Borders::ALL))
                    .bar_width(bar_width)
                    .data(&data_dist);

                f.render_widget(bar_chart, f.size());
            })
            .unwrap();
    }
}
