use num_complex::Complex;
use std::io::{stdout, Stdout};
use tui::{backend::CrosstermBackend, Terminal};

pub struct Display {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
        }
    }

    pub fn update(&self, data: &[Complex<f32>]) {
        let data_dist = data
            .iter()
            .map(|x| x.re * x.re + x.im * x.im)
            .collect::<Vec<f32>>();

        let mut max = data_dist[0];

        for d in &data_dist {
            if *d > max {
                max = *d;
            }
        }

        max = max.sqrt();

        for i in 0..data.len() {}
    }
}
