use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::blocking::Client;
use reqwest::Error;

pub type Parameters<'a> = HashMap<&'a str, &'a str>;
pub type _Response<T> = Result<Response<T>, reqwest::Error>;

pub struct NikelAPI {
    client: Client
}

impl NikelAPI {
    pub fn new() -> NikelAPI {
        NikelAPI {
            client: Client::new()
        }
    }

    fn encode_params(params: Parameters) -> String {
        params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<String>>()
        .join("&")
    } 

    fn get_url(endpoint: String, params: Parameters) -> String {
        format!("https://nikel.ml/api/{}?{}", endpoint, NikelAPI::encode_params(params))
    }

    fn get<T: DeserializeOwned>(&self, endpoint: String, params: Parameters) -> Result<Response<T>, reqwest::Error> {
        self.client.get(&NikelAPI::get_url(endpoint, params)).send()?.json::<Response<T>>()
    }

    pub fn courses(&self, params: Parameters) -> Result<Response<Course>, Error> {
        self.get("courses".to_owned(), params)
    }
    pub fn textbooks(&self, params: Parameters) -> Result<Response<Textbook>, Error> {
        self.get("textbooks".to_owned(), params)
    }
    pub fn exams(&self, params: Parameters) -> Result<Response<Exam>, Error> {
        self.get("exams".to_owned(), params)
    }
    pub fn evals(&self, params: Parameters) -> Result<Response<Eval>, Error> {
        self.get("evals".to_owned(), params)
    }
    pub fn food(&self, params: Parameters) -> Result<Response<Food>, Error> {
        self.get("food".to_owned(), params)
    }
    pub fn services(&self, params: Parameters) -> Result<Response<Service>, Error> {
        self.get("services".to_owned(), params)
    }
    pub fn buildings(&self, params: Parameters) -> Result<Response<Building>, Error> {
        self.get("buildings".to_owned(), params)
    }
    pub fn parking(&self, params: Parameters) -> Result<Response<Parking>, Error> {
        self.get("parking".to_owned(), params)
    }

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exam{}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eval{}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Food{}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service{}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building{}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parking{}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub response: Vec<T>,
    pub status_code: Option<i64>,
    pub status_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub division: Option<String>,
    pub department: Option<String>,
    pub prerequisites: Option<String>,
    pub corequisites: Option<String>,
    pub exclusions: Option<String>,
    pub recommended_preparation: Option<String>,
    pub level: Option<String>,
    pub campus: Option<String>,
    pub term: Option<String>,
    pub arts_and_science_breadth: Option<String>,
    pub arts_and_science_distribution: Option<String>,
    pub utm_distribution: Option<String>,
    pub utsc_breadth: Option<String>,
    pub apsc_electives: Option<String>,
    pub meeting_sections: Vec<CoursesMeetingSection>,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoursesMeetingSection {
    pub code: Option<String>,
    pub instructors: Vec<Option<String>>,
    pub times: Vec<Time>,
    pub size: Option<i64>,
    pub enrollment: Option<i64>,
    pub waitlist_option: Option<bool>,
    pub delivery: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub day: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub duration: Option<i64>,
    pub location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Textbook {
    pub id: Option<String>,
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub edition: Option<i64>,
    pub author: Option<String>,
    pub image: Option<String>,
    pub price: Option<f64>,
    pub url: Option<String>,
    pub courses: Vec<Option<TextbooksCourse>>,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextbooksCourse {
    pub id: Option<String>,
    pub code: Option<String>,
    pub requirement: Option<String>,
    pub meeting_sections: Vec<Option<TextbooksMeetingSection>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextbooksMeetingSection {
    pub code: Option<String>,
    pub instructors: Vec<Option<String>>,
}