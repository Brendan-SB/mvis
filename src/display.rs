use crate::{config::Config, PROGRAM_NAME};
use num_complex::Complex;
use std::io::{stdout, Stdout};
use tui::{
    backend::TermionBackend,
    style::Style,
    widgets::{BarChart, Block, Borders},
    Terminal,
};

pub struct Display<'a> {
    config: &'a Config,
    terminal: Terminal<TermionBackend<Stdout>>,
    bar_style: Style,
}

impl<'a> Display<'a> {
    pub fn new(config: &'a Config) -> anyhow::Result<Self> {
        let mut terminal = Terminal::new(TermionBackend::new(stdout()))?;

        terminal.clear()?;

        Ok(Self {
            config,
            terminal,
            bar_style: config.style.to_tui_style()?,
        })
    }

    fn calculate_offset(data_dist_len: f64, bar_width: f64, terminal_width: f64) -> f64 {
        if terminal_width > bar_width && terminal_width > 0_f64 && data_dist_len > 0_f64 {
            ((data_dist_len + bar_width / data_dist_len) / terminal_width).round() + 1_f64
        } else {
            1_f64
        }
    }

    fn create_bars(data: &[Complex<f64>], bar_width: f64, terminal_width: f64) -> Vec<u64> {
        let mut data_dist_reformed = Vec::new();

        {
            let data_dist = data
                .iter()
                .map(|x| x.re * x.re + x.im * x.im)
                .filter(|x| x.round() >= 0.0)
                .collect::<Vec<_>>();

            let offset = Self::calculate_offset(data_dist.len() as f64, bar_width, terminal_width);

            for i in (0..data_dist.len() - offset as usize).step_by(offset as usize) {
                let mut sum = 0_f64;

                for j in data_dist.iter().skip(i).take(offset as usize) {
                    sum += *j;
                }

                let value = (sum / offset).round() as u64;

                data_dist_reformed.push(value);
            }
        }

        data_dist_reformed
    }

    pub fn update(&mut self, data: &[Complex<f64>]) -> anyhow::Result<()> {
        let bar_width = self.config.bar_width;
        let terminal_width = self.terminal.size()?.width;
        let bar_style = self.bar_style;

        self.terminal.draw(move |f| {
            let data_dist = Self::create_bars(data, bar_width as f64, terminal_width as f64);
            let plot = data_dist
                .iter()
                .cloned()
                .map(|i| ("", (i as f64 / (i as f64).sqrt()) as u64))
                .collect::<Vec<_>>();
            let bar_chart = BarChart::default()
                .block(Block::default().title(PROGRAM_NAME).borders(Borders::ALL))
                .bar_width(bar_width)
                .style(bar_style)
                .data(&plot);

            f.render_widget(bar_chart, f.size());
        })?;

        Ok(())
    }
}
