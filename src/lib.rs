use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use reqwest::blocking::Client;
use reqwest::Error;

pub type Parameters<'a> = HashMap<&'a str, &'a str>;

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

// [ BEGIN DATA TYPES ]

// [ RESPONSE STRUCT ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub response: Vec<T>,
    pub status_code: Option<i64>,
    pub status_message: Option<String>,
}

// [ COURSES ]

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
    pub instructors: Vec<String>,
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

// [ TEXTBOOKS ]

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
    pub courses: Vec<TextbooksCourse>,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextbooksCourse {
    pub id: Option<String>,
    pub code: Option<String>,
    pub requirement: Option<String>,
    pub meeting_sections: Vec<TextbooksMeetingSection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextbooksMeetingSection {
    pub code: Option<String>,
    pub instructors: Vec<String>,
}

// [ EXAMS ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exam {
    pub id: Option<String>,
    pub course_id: Option<String>,
    pub course_code: Option<String>,
    pub campus: Option<String>,
    pub date: Option<String>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub duration: Option<i64>,
    pub sections: Vec<ExamSection>,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExamSection {
    pub lecture_code: Option<String>,
    pub split: Option<String>,
    pub location: Option<String>,
}

// [ EVALS ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eval {
    pub id: Option<String>,
    pub name: Option<String>,
    pub campus: Option<String>,
    pub terms: Vec<EvalTerm>,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalTerm {
    pub term: Option<String>,
    pub lectures: Vec<EvalLecture>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalLecture {
    pub lecture_code: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub s1: Option<f64>,
    pub s2: Option<f64>,
    pub s3: Option<f64>,
    pub s4: Option<f64>,
    pub s5: Option<f64>,
    pub s6: Option<f64>,
    pub invited: Option<i64>,
    pub responses: Option<i64>,
}

// [ FOOD ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Food {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub campus: Option<String>,
    pub address: Option<String>,
    pub coordinates: Coordinates,
    pub hours: Hours,
    pub image: Option<String>,
    pub url: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub attributes: Vec<String>,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hours {
    pub sunday: Sunday,
    pub monday: Monday,
    pub tuesday: Tuesday,
    pub wednesday: Wednesday,
    pub thursday: Thursday,
    pub friday: Friday,
    pub saturday: Saturday,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sunday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Monday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tuesday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wednesday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thursday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Friday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Saturday {
    pub closed: Option<bool>,
    pub open: Option<i64>,
    pub close: Option<i64>,
}

// [ SERVICES ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<String>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub building_id: Option<String>,
    pub description: Option<String>,
    pub campus: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
    pub coordinates: Option<Coordinates>,
    pub tags: Option<String>,
    pub attributes: Vec<String>,
    pub last_updated: Option<String>,
}

// [ BUILDINGS ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub id: Option<String>,
    pub code: Option<String>,
    pub tags: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub address: BuildingAddress,
    pub coordinates: Coordinates,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingAddress {
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal: Option<String>,
}

// [ PARKING ]

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parking {
    pub id: Option<String>,
    pub name: Option<String>,
    pub alias: Option<String>,
    pub building_id: Option<String>,
    pub description: Option<String>,
    pub campus: Option<String>,
    pub address: Option<String>,
    pub coordinates: Coordinates,
    pub last_updated: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
