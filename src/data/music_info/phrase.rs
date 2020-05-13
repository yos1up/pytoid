use pyo3::class::{PyNumberProtocol, PyObjectProtocol};
use pyo3::prelude::{pyclass, pyproto, PyObject, PyResult};

use toid::data::music_info::phrase;

use super::super::super::high_layer_trial::marge;

#[pyclass]
#[derive(Clone)]
pub struct Phrase {
    pub phrase: phrase::Phrase,
}

#[pyproto]
impl PyObjectProtocol for Phrase {
    fn __str__(&self) -> PyResult<String> {
        let s = serde_json::to_string(&self.phrase).unwrap();
        Ok(s)
    }
}

#[pyproto]
impl PyNumberProtocol for Phrase {
    fn __mul__(lhs: Self, rhs: Self) -> PyResult<Self> {
        Ok(marge(lhs, rhs))
    }
}
