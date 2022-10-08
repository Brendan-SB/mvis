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

    fn calculate_offset(data_dist_len: f32, bar_width: f32, terminal_width: f32) -> f32 {
        if terminal_width > bar_width && terminal_width > 0_f32 {
            ((1_f32 + data_dist_len + bar_width * data_dist_len) / terminal_width).round()
        } else {
            1_f32
        }
    }

    fn create_bars(data: &[Complex<f32>], bar_width: f32, terminal_width: f32) -> Vec<(&str, u64)> {
        let mut data_dist_reformed = Vec::new();

        {
            let data_dist = data
                .iter()
                .map(|x| (x.re * x.re + x.im * x.im).round())
                .collect::<Vec<f32>>();

            let offset = Self::calculate_offset(data_dist.len() as f32, bar_width, terminal_width);

            for i in (0..data_dist.len() - offset as usize).step_by(offset as usize) {
                let mut sum = 0_f32;

                for j in i..i + (offset as usize) {
                    sum += data_dist[j];
                }

                data_dist_reformed.push(("", (sum / offset).round() as u64));
            }
        }

        data_dist_reformed
    }

    pub fn update(&mut self, data: &[Complex<f32>]) -> anyhow::Result<()> {
        let bar_width = self.config.bar_width;
        let terminal_width = self.terminal.size()?.width;
        let bar_style = self.bar_style.clone();

        self.terminal.draw(move |f| {
            let data_dist = Self::create_bars(data, bar_width as f32, terminal_width as f32);

            let bar_chart = BarChart::default()
                .block(Block::default().title(PROGRAM_NAME).borders(Borders::ALL))
                .bar_width(bar_width)
                .style(bar_style)
                .data(&data_dist);

            f.render_widget(bar_chart, f.size());
        })?;

        Ok(())
    }
}
