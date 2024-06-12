use crate::{config::Config, PROGRAM_NAME};
use num_complex::Complex;
use rayon::prelude::*;
use std::io::{stdout, Stdout};
use tui::{
    backend::TermionBackend,
    style::Style,
    widgets::{BarChart, Block, Borders},
    Terminal,
};

pub struct Display {
    terminal: Terminal<TermionBackend<Stdout>>,
    bar_style: Style,
}

impl Display {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        let mut terminal = Terminal::new(TermionBackend::new(stdout()))?;

        terminal.clear()?;

        Ok(Self {
            terminal,
            bar_style: config.style.to_tui_style()?,
        })
    }

    fn calculate_offset(data_dist_len: f64, terminal_width: f64) -> f64 {
        if terminal_width > 0_f64 && data_dist_len > 0_f64 {
            ((data_dist_len + 1_f64 / data_dist_len) / terminal_width).round()
        } else {
            1_f64
        }
    }

    fn create_bars(data: &[Complex<f64>], terminal_width: f64) -> Vec<(&str, u64)> {
        let data_dist = data
            .par_iter()
            .map(|x| x.re * x.re + x.im * x.im)
            .filter(|x| x.round() >= 0.0)
            .collect::<Vec<_>>();

        let offset = Self::calculate_offset(data_dist.len() as f64, terminal_width);

        (0..data_dist.len() - offset as usize)
            .step_by(offset as usize)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|i| {
                let sum: f64 = data_dist.iter().skip(i).take(offset as usize).sum();
                let value = (sum / offset).sqrt().round() as u64;

                ("", value)
            })
            .collect::<Vec<_>>()
    }

    pub fn update(&mut self, data: &[Complex<f64>]) -> anyhow::Result<()> {
        let terminal_width = self.terminal.size()?.width;
        let bar_style = self.bar_style;

        self.terminal.draw(move |f| {
            let plot = Self::create_bars(data, terminal_width as f64);
            let bar_chart = BarChart::default()
                .block(Block::default().title(PROGRAM_NAME).borders(Borders::ALL))
                .style(bar_style)
                .data(&plot);

            f.render_widget(bar_chart, f.size());
        })?;

        Ok(())
    }
}
