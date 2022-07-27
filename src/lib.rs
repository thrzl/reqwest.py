use pyo3::prelude::*;
use pyo3::{create_exception, exceptions::PyException};
use reqwest::blocking::get as r_get;
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

mod client;
mod response;
use response::Response;

create_exception!(reqwest, HTTPError, PyException);

#[pyfunction]
fn get(url: &str) -> PyResult<String> {
    Ok(r_get(url).unwrap().text().unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn reqwest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_class::<client::Client>()?;
    m.add_class::<Response>()?;
    Ok(())
}
