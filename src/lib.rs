pub mod core;
pub mod python;
pub mod utils;

use pyo3::types::PyModuleMethods;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use pyo3::{wrap_pyfunction, Bound};

use crate::python::{get_backend_info, PyWgpuRenderer};

#[pymodule]
#[pyo3(name = "lsd")]
fn wgpu_integration_py(_py: Python, module: Bound<'_, PyModule>) -> PyResult<()> {
    init_logging();

    module.add_class::<PyWgpuRenderer>()?;
    module.add_function(wrap_pyfunction!(get_backend_info, &module)?)?;

    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__author__", "Veaksam Metra <exactlythatguy@gmail.com>")?;
    module.add(
        "__description__",
        "PyQt6 integration for wgpu graphics library",
    )?;

    Ok(())
}

fn init_logging() {
    #[cfg(debug_assertions)]
    {
        // В debug режиме включаем подробное логирование
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }

    #[cfg(not(debug_assertions))]
    {
        // В release режиме только ошибки
        std::env::set_var("RUST_LOG", "error");
        env_logger::init();
    }

    log::info!(
        "wgpu_integration v{} initialized",
        env!("CARGO_PKG_VERSION")
    );
}

/// Тесты интеграции
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
    }

    #[test]
    fn test_error_handling() {
    }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".ctors")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XCU")]
static INIT: unsafe extern "C" fn() = {
    #[cfg_attr(target_os = "linux", link_section = ".text.startup")]
    unsafe extern "C" fn init() {
        // Инициализация при загрузке библиотеки
    }
    init
};
