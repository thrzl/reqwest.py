use miniserde::json::from_str;
use pyo3::prelude::*;
use pyo3::{create_exception, exceptions::PyException};
use reqwest::{
    blocking::{get as r_get},
};
use std::{collections::HashMap};
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

mod client;
create_exception!(reqwest, HTTPError, PyException);

#[pyfunction]
fn get(url: &str) -> PyResult<String> {
    Ok(r_get(url).unwrap().text().unwrap())
}


#[pyclass]
struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

#[pymethods]
impl Response {
    #[new]
    fn new(status: u16, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    fn json(&self) -> PyResult<HashMap<String, String>> {
        Ok(from_str(&self.body).unwrap())
    }

    fn text(&self) -> PyResult<String> {
        Ok(self.body.clone())
    }

    fn raise_for_status(&self) -> PyResult<Option<PyErr>> {
        if 400 <= self.status && self.status < 500 {
            Err(PyErr::new::<HTTPError, _>(format!(
                "HTTP error {}",
                self.status
            )))
        } else if 500 <= self.status && self.status < 600 {
            Err(PyErr::new::<HTTPError, _>(format!(
                "HTTP error {}",
                self.status
            )))
        } else {
            Ok(None)
        }
    }

    fn headers(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.headers.clone())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn reqwest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_class::<client::Client>()?;
    m.add_class::<Response>()?;
    Ok(())
}
