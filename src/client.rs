use pyo3::prelude::*;
use reqwest::blocking::{Client as r_Client};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Method;
use std::{collections::HashMap, str::FromStr};
use crate::{Response, APP_USER_AGENT};

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
pub struct Client {
    r_client: r_Client,
}

#[pymethods]
impl Client {
    #[new]
    fn new(h: Option<HashMap<String, String>>) -> Self {
        let headers = h.unwrap_or(HashMap::new());
        let hmap = dict_to_headers(headers);
        Self {
            r_client: r_Client::builder()
                .default_headers(hmap)
                .user_agent(APP_USER_AGENT)
                .build()
                .unwrap(),
        }
    }

    fn request(
        &self,
        method: &str,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Response {
        let r = self
            .r_client
            .request(Method::from_bytes(method.as_bytes()).unwrap(), url)
            .headers(dict_to_headers(headers.unwrap_or(HashMap::new())))
            .send()
            .unwrap();
        let mut h: HashMap<String, String> = HashMap::with_capacity(r.headers().len());
        for (key, value) in r.headers().iter() {
            h.insert(key.to_string(), value.to_str().unwrap().to_string());
        }
        Response {
            status: r.status().as_u16(),
            headers: h,
            body: r.text().unwrap(),
        }
    }

    fn get(&self, url: &str, headers: Option<HashMap<String, String>>) -> PyResult<Response> {
        Ok(self.request("GET", url, headers))
    }
}
