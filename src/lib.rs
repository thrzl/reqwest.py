use pyo3::prelude::*;
use reqwest::{blocking::{get as r_get, Client as r_Client}, header::{HeaderMap, HeaderName, HeaderValue}};
use std::collections::HashMap;

#[pyfunction]
fn get(url: &str) -> PyResult<String> {
    Ok(r_get(url).unwrap().text().unwrap())
}

#[pyclass]
struct Client {
    r_client: r_Client,
    headers: HeaderMap,
}

#[pymethods]
impl Client {
    #[new]
    fn new(h: Option<HashMap<String, String>>) -> Self {
        let headers = h.unwrap_or(HashMap::new());
        let mut hmap: HeaderMap = HeaderMap::with_capacity(headers.capacity());
        for (k, v) in headers {
            hmap.insert(HeaderName::from_static(&k), HeaderValue::from_static(&v));
        }
        Self {
            r_client: r_Client::new(),
            headers: hmap,
        }
    }

    fn request(&self, method: &str, url: &str, headers: Option<HashMap<String, String>>) -> String {
        self.r_client.execute(
            self.r_client.request(method, url).headers(headers).build().unwrap()
        )
        .unwrap()
        .text()
        .unwrap()
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