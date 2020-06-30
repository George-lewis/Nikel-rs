mod types;

use types::*;

use reqwest::blocking::Client;

pub struct NikelAPI {
    client: Client
}

impl NikelAPI {
    pub fn new() -> NikelAPI {
        NikelAPI {
            client: Client::new()
        }
    }

    fn encode_params(params: std::collections::HashMap<&str, &str>) -> String {
        params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&")
    } 

    fn get_url(endpoint: String, params: std::collections::HashMap<&str, &str>) -> String {
        format!("https://nikel.ml/api/{}?{}", endpoint, NikelAPI::encode_params(params))
    }

    pub fn courses(&self, params: std::collections::HashMap<&str, &str>) -> Result<Response<Course>, reqwest::Error> {
        self.client.get(&NikelAPI::get_url("courses".to_owned(), params)).send()?.json()
    }

    pub fn textbooks(&self, params: std::collections::HashMap<&str, &str>) -> Result<Response<Textbook>, reqwest::Error> {
        self.client.get(&NikelAPI::get_url("textbooks".to_owned(), params)).send()?.json()
    }

}