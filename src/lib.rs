use pyo3::prelude::*;
use reqwest::{
    blocking::{get as r_get, Client as r_Client},
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
use std::{collections::HashMap, str::FromStr};
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);
use miniserde::json::from_str;

#[pyfunction]
fn get(url: &str) -> PyResult<String> {
    Ok(r_get(url).unwrap().text().unwrap())
}

fn dict_to_headers(dict: HashMap<String, String>) -> HeaderMap {
    let mut headers = HeaderMap::with_capacity(dict.len());
    for (key, value) in dict {
        headers.insert(
            HeaderName::from_str(&key).unwrap(),
            HeaderValue::from_str(&value).unwrap(),
        );
    }
    headers
}

#[pyclass]
struct Response {
    status: u16,
    headers: HeaderMap,
    body: String,
}

#[pymethods]
impl Response {
    #[new]
    fn new(status: u16, headers: HashMap<String, String>, body: String) -> Self {
        Self {
            status,
            headers: dict_to_headers(headers),
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
            Err(PyErr::new::<PyValueError, _>(format!(
                "HTTP error {}",
                self.status
            )))
        } else {
            Ok(None)
        }
    }
}

#[pyclass]
struct Client {
    r_client: r_Client,
}

#[pymethods]
impl Client {
    #[new]
    fn new(h: Option<HashMap<String, String>>) -> Self {
        let headers = h.unwrap_or(HashMap::new());
        let hmap = dict_to_headers(headers);
        Self {
            r_client: r_Client::builder().default_headers(hmap).user_agent(APP_USER_AGENT).build().unwrap(),
        }
    }

    fn request(&self, method: &str, url: &str, headers: Option<HashMap<String, String>>) -> String {
            self.r_client
            .request(Method::from_bytes(method.as_bytes()).unwrap(), url)
            .headers(
                dict_to_headers(headers.unwrap_or(HashMap::new())),
            )
            .send()
            .unwrap()
            .text()
            .unwrap().trim().to_string()
    }

    fn get(&self, url: &str, headers: Option<HashMap<String, String>>) -> PyResult<String> {
        Ok(self.request("GET", url, headers))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn reqwest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_class::<Client>()?;
    Ok(())
}
