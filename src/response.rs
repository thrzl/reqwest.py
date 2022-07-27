use crate::HTTPError;
use miniserde::json::from_str;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub reason: String,
}

#[pymethods]
impl Response {
    #[new]
    fn new(status: u16, headers: HashMap<String, String>, body: String, reason: String) -> Self {
        Self {
            status,
            headers,
            body,
            reason,
        }
    }

    fn json(&self) -> PyResult<HashMap<String, String>> {
        Ok(from_str(&self.body).unwrap())
    }

    fn text(&self) -> PyResult<String> {
        Ok(self.body.clone())
    }

    fn raise_for_status(&self) -> PyResult<Option<PyErr>> {
        if 400 <= self.status && self.status < 600 {
            Err(PyErr::new::<HTTPError, _>(format!("{} {}", self.status, self.reason.clone())))
        } else {
            Ok(None)
        }
    }

    fn headers(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.headers.clone())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<Request status={}>", self.status))
    }
}
