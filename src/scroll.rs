use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use serde::Deserialize;
use serde_json::value::RawValue;

use crate::auth::AuthString;

#[derive(Debug, Deserialize)]
pub struct SearchHitsMetadata {
    hits: Vec<Box<RawValue>>,
    total: SearchHitsTotal
}

#[derive(Deserialize, Debug)]
pub struct SearchHitsTotal {
    value: u32
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    _scroll_id: String,
    hits: SearchHitsMetadata,
}

#[derive(Debug)]
pub struct Scroll {
    host: String,
    index: String,
    auth: Option<AuthString>,
    query_body: String,
    result_size: u32,
    page_number: u32,
    buffer_pointer: usize,
    scroll_pointer: Option<String>,
    scroll_buffer: Vec<Box<RawValue>>,
}

impl Scroll {
    pub fn new(host: String, index: String, auth: Option<AuthString>, query_body: String) -> Self {
        let _ = vec![1, 2, 3].iter();

        Self {
            host,
            index,
            auth,
            query_body,
            result_size: 0,
            page_number: 0,
            buffer_pointer: 0,
            scroll_pointer: None,
            scroll_buffer: Vec::new(),
        }
    }

    fn start_scroll(&mut self) -> () {
        let endpoint = format!("{}/{}/_search?scroll=1m", &self.host, &self.index);

        let response = post(&endpoint, &self.auth)
            .body(self.query_body.clone())
            .send();

        match response.and_then(|result| result.json::<SearchResponse>()) {
            Ok(result) => {
                self.scroll_buffer = result.hits.hits;
                self.result_size = result.hits.total.value;
                self.scroll_pointer = Some(result._scroll_id);
                self.buffer_pointer = 0;

                eprintln!("Expecting {} results", &self.result_size);
            }
            Err(err) => {
                dbg!(err);
                panic!("Uh oh!");
            }
        };
    }

    fn continue_scroll(&mut self) -> () {
        let endpoint = format!("{}/_search/scroll", &self.host);

        let response = post(&endpoint, &self.auth)
            .body(format!(
                "{{ \"scroll\": \"1m\", \"scroll_id\": \"{}\" }}",
                &self.scroll_pointer.as_ref().unwrap()
            ))
            .send();

        match response.and_then(|result| result.json::<SearchResponse>()) {
            Ok(result) => {
                self.scroll_buffer = result.hits.hits;
                self.scroll_pointer = Some(result._scroll_id);
                self.buffer_pointer = 0;
            }
            Err(err) => {
                dbg!(err);
                panic!("Uh oh!");
            }
        };
    }
}

impl Iterator for Scroll {
    type Item = Box<RawValue>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scroll_buffer.len() == 0 || self.buffer_pointer >= self.scroll_buffer.len() {
            self.page_number += 1;

            eprintln!("Scroll Page: {}", &self.page_number);

            if let Some(_scroll_id) = &self.scroll_pointer {
                self.continue_scroll();
            } else {
                self.start_scroll();
            };
        }

        if self.scroll_buffer.len() == 0 {
            return None;
        }

        self.buffer_pointer += 1;

        Some(self.scroll_buffer[self.buffer_pointer - 1].clone())
    }
}

fn post(endpoint: &str, auth: &Option<AuthString>) -> RequestBuilder {
    let client = Client::new();

    let client = client
        .post(endpoint)
        .header(CONTENT_TYPE, "application/json");

    match auth {
        Some(auth_string) => client.header(AUTHORIZATION, format!("{}", &auth_string)),
        None => client,
    }
}

pub fn scroll(host: String, index: String, auth: Option<AuthString>, query_body: String) -> Scroll {
    Scroll::new(host, index, auth, query_body)
}

