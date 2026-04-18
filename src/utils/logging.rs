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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_scope_timer_creation() {
        let timer = ScopeTimer::new("test_operation");
        assert_eq!(timer.name, "test_operation");
        assert!(timer.start.elapsed() < Duration::from_millis(10));
    }

    #[test]
    fn test_scope_timer_measures_time() {
        let timer = ScopeTimer::new("sleep_test");
        std::thread::sleep(Duration::from_millis(1));
        let elapsed = timer.start.elapsed();
        assert!(elapsed >= Duration::from_millis(1));
    }

    #[test]
    fn test_scope_timer_name_preservation() {
        let test_name = "complex_operation_123";
        let timer = ScopeTimer::new(test_name);
        assert_eq!(timer.name, test_name);
    }

    #[test]
    fn test_scope_timer_empty_name() {
        let timer = ScopeTimer::new("");
        assert_eq!(timer.name, "");
    }

    #[test]
    fn test_scope_timer_with_unicode() {
        let timer = ScopeTimer::new("операция_测试");
        assert_eq!(timer.name, "операция_测试");
    }

    #[test]
    fn test_scope_timer_logging_integration() {
        // Этот тест сложнее реализовать без захвата логов
        // Но можем проверить, что таймер не паникует при дропе
        {
            let _timer = ScopeTimer::new("integration_test");
            // Таймер будет дропнут в конце блока
        }
        // Если мы дошли до этой точки, значит drop прошел без паники
        assert!(true);
    }
}
