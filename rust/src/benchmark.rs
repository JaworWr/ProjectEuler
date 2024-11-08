macro_rules! benchmark {
    ($e:expr) => {
        {
            use std::time::Instant;

            let start_time = Instant::now();
            let result = $e;
            let duration = Instant::now() - start_time;
            eprintln!("{} took {:.6} s", stringify!($e), duration.as_micros() as f64 / 1000000.);
            result
        }
    };
}

pub(crate) use benchmark;

use std::time::Instant;            

#[derive(Debug, Clone)]
pub struct ProgressLogger<T> {
    ctr: usize,
    step: usize,
    total: String,
    name: String,
    start_time: Option<Instant>,
    inner: T,
}

impl<T: Iterator> ProgressLogger<T> {
    pub fn new(inner: T, step: usize, name: String) -> Self {
        let total = match inner.size_hint().1 {
            Some(x) => x.to_string(),
            None => "???".to_string(),
        };
        Self { 
            ctr: 0, 
            step, 
            total,
            name,
            start_time: None,
            inner,
        }
    }

    fn ms_since_start(&self) -> f64 {
        let now = Instant::now();
        (now - self.start_time.unwrap()).as_millis() as f64 / 1000.
    }
}

impl<T: Iterator> Iterator for ProgressLogger<T> {
    type Item = T::Item;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.start_time.get_or_insert_with(Instant::now);
        let result = self.inner.next();
        if result.is_some() {
            self.ctr += 1;
            if self.ctr % self.step == 0 {
                let since_start = self.ms_since_start();
                eprintln!("{}: processed {:>w$}/{}, took {:>7.3} s", 
                    self.name, self.ctr, self.total, since_start, w = self.total.len());
            }
        }
        else {
            let since_start = self.ms_since_start();
            eprintln!("{}: done, took {:.3} s", self.name, since_start);
        }
        result
    }
}

pub trait Progress: Iterator + Sized {
    fn log_progress(self, step: usize, name: impl ToString) -> ProgressLogger<Self>;
}

impl<T: Iterator> Progress for T {
    fn log_progress(self, step: usize, name: impl ToString) -> ProgressLogger<Self> {
        ProgressLogger::new(self, step, name.to_string())
    }
}
