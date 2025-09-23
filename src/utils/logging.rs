use env_logger::Builder;
use log::{info, LevelFilter};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .try_init()?;

    info!("Logging system initialized");
    Ok(())
}

pub struct ScopeTimer {
    start: std::time::Instant,
    name: String,
}

impl ScopeTimer {
    pub fn new(name: &str) -> Self {
        Self {
            start: std::time::Instant::now(),
            name: name.to_string(),
        }
    }
}

impl Drop for ScopeTimer {
    fn drop(&mut self) {
        info!("{} took {:?}", self.name, self.start.elapsed());
    }
}
