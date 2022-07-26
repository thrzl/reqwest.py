use pyo3::prelude::*;
use reqwest::blocking::{get as r_get, Client as r_Client};

#[pyfunction]
fn get(url: &str) -> PyResult<String> {
    Ok(r_get(url).unwrap().text().unwrap())
}

#[pyclass]
struct Client {
    r_client: r_Client,
}

#[pymethods]
impl Client {
    #[new]
    fn new() -> Self {
        Self {
            r_client: r_Client::new(),
        }
    }
    fn get(&self, url: &str) -> PyResult<String> {
        Ok(self.r_client.get(url).send().unwrap().text().unwrap())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn reqwest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_class::<Client>()?;
    Ok(())
}