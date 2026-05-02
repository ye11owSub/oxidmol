use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::PyResult;
use pyo3::{pyclass, pymethods, PyErr};

use crate::core::Molecule;
use crate::utils::LoadError;

impl From<LoadError> for PyErr {
    fn from(error: LoadError) -> Self {
        PyRuntimeError::new_err(error.to_string())
    }
}

#[pyclass(name = "Molecule")]
pub struct PyMolecule {
    pub(crate) inner: Molecule,
}

#[pymethods]
impl PyMolecule {
    #[staticmethod]
    #[pyo3(name = "from_cif")]
    pub fn py_from_cif(path: String) -> PyResult<Self> {
        let inner = Molecule::load(&path)?;
        Ok(Self { inner })
    }

    #[staticmethod]
    #[pyo3(name = "from_pdb")]
    pub fn py_from_pdb(path: String) -> PyResult<Self> {
        let inner = Molecule::load(&path)?;
        Ok(Self { inner })
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    /// Parse PDB or mmCIF from a string. Format auto-detected from first line.
    pub fn py_from_str(content: String) -> PyResult<Self> {
        let inner = Molecule::load_str(&content)?;
        Ok(Self { inner })
    }

    pub fn name(&self) -> Option<String> {
        self.inner.name.clone()
    }
}
