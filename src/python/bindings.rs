use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::{Py, PyErr, PyResult, Python};
use pyo3::types::{PyDict, PyDictMethods};
use pyo3::{pyclass, pyfunction, pymethods};

use crate::core::renderer::State;
use crate::python::molecule::PyMolecule;
use crate::utils::error::WgpuError;

#[pyfunction]
pub fn get_backend_info() -> PyResult<Py<PyDict>> {
    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        dict.set_item("backend", "wgpu")?;
        dict.set_item("version", env!("CARGO_PKG_VERSION"))?;
        dict.set_item("supported_apis", vec!["vulkan", "metal", "dx12", "gl"])?;
        Ok(dict.into())
    })
}

#[pyfunction]
pub fn create_shader_from_file(path: String) -> PyResult<String> {
    std::fs::read_to_string(&path)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to read shader file: {}", e)))
}

impl From<WgpuError> for PyErr {
    fn from(error: WgpuError) -> Self {
        PyRuntimeError::new_err(error.to_string())
    }
}

#[pyclass]
pub struct WgpuRenderer {
    inner: State<'static>,
}

#[pymethods]
impl WgpuRenderer {
    #[new]
    #[pyo3(signature = (window_handle, *, width = 800, height = 600))]
    pub fn new(window_handle: usize, width: u32, height: u32) -> PyResult<Self> {
        let inner = pollster::block_on(State::new(window_handle, width, height));

        Ok(Self { inner })
    }

    #[pyo3(name = "render")]
    pub fn py_render(&self) -> PyResult<()> {
        self.inner
            .render()
            .map_err(|e| PyRuntimeError::new_err(format!("Render failed: {}", e)))
    }
    /// Load a PDB or mmCIF file (.pdb / .cif) and upload atoms to the GPU.
    pub fn load_molecule(&mut self, path: String) -> PyResult<()> {
        self.inner
            .load_molecule(&path)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// Upload an already-parsed Molecule to the GPU.
    pub fn load_molecule_obj(&mut self, mol: &PyMolecule) -> PyResult<()> {
        self.inner
            .upload_molecule(&mol.inner)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    #[getter]
    /// Get current width
    pub fn width(&self) -> u32 {
        self.inner.size.0
    }

    #[getter]
    /// Get current height
    pub fn height(&self) -> u32 {
        self.inner.size.1
    }
}
