use reqwest::blocking::Client;

use serde::{Serialize, Deserialize};

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct ClassResponse {
//     pub response: Vec<Class>,
//     pub status_code: Option<i64>,
//     pub status_message: Option<String>,
// }

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
    pub times: Vec<CoursesTime>,
    pub size: Option<i64>,
    pub enrollment: Option<i64>,
    pub waitlist_option: Option<bool>,
    pub delivery: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoursesTime {
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