use std::vec::Vec;
use std::fmt;
use std::fmt::Display;

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use reqwest::blocking::Client;
use reqwest::Error;

pub type Parameters<'a> = Vec<(&'a str, &'a str)>;

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

    fn get<T: DeserializeOwned>(&self, endpoint: String, params: Parameters) -> Result<Response<T>, Error> {
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

// For the sake of printing
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct OptionWrapper<T>(Option<T>);

impl<T: Display> fmt::Debug for OptionWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output: String;
        if self.0.is_none() {
            output = "Unknown".to_string()
        } else {
            output = format!("{}", self.0.as_ref().unwrap())
        }
        write!(f, "{}", output)
    }
}

// [ RESPONSE STRUCT ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub response: Vec<T>,
    pub status_code: OptionWrapper<i64>,
    pub status_message: OptionWrapper<String>,
}

// [ COURSES ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: OptionWrapper<String>,
    pub code: OptionWrapper<String>,
    pub name: OptionWrapper<String>,
    pub description: OptionWrapper<String>,
    pub division: OptionWrapper<String>,
    pub department: OptionWrapper<String>,
    pub prerequisites: OptionWrapper<String>,
    pub corequisites: OptionWrapper<String>,
    pub exclusions: OptionWrapper<String>,
    pub recommended_preparation: OptionWrapper<String>,
    pub level: OptionWrapper<String>,
    pub campus: OptionWrapper<String>,
    pub term: OptionWrapper<String>,
    pub arts_and_science_breadth: OptionWrapper<String>,
    pub arts_and_science_distribution: OptionWrapper<String>,
    pub utm_distribution: OptionWrapper<String>,
    pub utsc_breadth: OptionWrapper<String>,
    pub apsc_electives: OptionWrapper<String>,
    pub meeting_sections: Vec<CoursesMeetingSection>,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CoursesMeetingSection {
    pub code: OptionWrapper<String>,
    pub instructors: Vec<String>,
    pub times: Vec<Time>,
    pub size: OptionWrapper<i64>,
    pub enrollment: OptionWrapper<i64>,
    pub waitlist_option: OptionWrapper<bool>,
    pub delivery: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub day: OptionWrapper<String>,
    pub start: OptionWrapper<i64>,
    pub end: OptionWrapper<i64>,
    pub duration: OptionWrapper<i64>,
    pub location: OptionWrapper<String>,
}

// [ TEXTBOOKS ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Textbook {
    pub id: OptionWrapper<String>,
    pub isbn: OptionWrapper<String>,
    pub title: OptionWrapper<String>,
    pub edition: OptionWrapper<i64>,
    pub author: OptionWrapper<String>,
    pub image: OptionWrapper<String>,
    pub price: OptionWrapper<f64>,
    pub url: OptionWrapper<String>,
    pub courses: Vec<TextbooksCourse>,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TextbooksCourse {
    pub id: OptionWrapper<String>,
    pub code: OptionWrapper<String>,
    pub requirement: OptionWrapper<String>,
    pub meeting_sections: Vec<TextbooksMeetingSection>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TextbooksMeetingSection {
    pub code: OptionWrapper<String>,
    pub instructors: Vec<String>,
}

// [ EXAMS ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Exam {
    pub id: OptionWrapper<String>,
    pub course_id: OptionWrapper<String>,
    pub course_code: OptionWrapper<String>,
    pub campus: OptionWrapper<String>,
    pub date: OptionWrapper<String>,
    pub start: OptionWrapper<i64>,
    pub end: OptionWrapper<i64>,
    pub duration: OptionWrapper<i64>,
    pub sections: Vec<ExamSection>,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExamSection {
    pub lecture_code: OptionWrapper<String>,
    pub split: OptionWrapper<String>,
    pub location: OptionWrapper<String>,
}

// [ EVALS ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Eval {
    pub id: OptionWrapper<String>,
    pub name: OptionWrapper<String>,
    pub campus: OptionWrapper<String>,
    pub terms: Vec<EvalTerm>,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EvalTerm {
    pub term: OptionWrapper<String>,
    pub lectures: Vec<EvalLecture>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EvalLecture {
    pub lecture_code: OptionWrapper<String>,
    pub firstname: OptionWrapper<String>,
    pub lastname: OptionWrapper<String>,
    pub s1: OptionWrapper<f64>,
    pub s2: OptionWrapper<f64>,
    pub s3: OptionWrapper<f64>,
    pub s4: OptionWrapper<f64>,
    pub s5: OptionWrapper<f64>,
    pub s6: OptionWrapper<f64>,
    pub invited: OptionWrapper<i64>,
    pub responses: OptionWrapper<i64>,
}

// [ FOOD ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub id: OptionWrapper<String>,
    pub name: OptionWrapper<String>,
    pub description: OptionWrapper<String>,
    pub tags: OptionWrapper<String>,
    pub campus: OptionWrapper<String>,
    pub address: OptionWrapper<String>,
    pub coordinates: Coordinates,
    pub hours: Hours,
    pub image: OptionWrapper<String>,
    pub url: OptionWrapper<String>,
    pub twitter: OptionWrapper<String>,
    pub facebook: OptionWrapper<String>,
    pub attributes: Vec<String>,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Hours {
    pub sunday: Sunday,
    pub monday: Monday,
    pub tuesday: Tuesday,
    pub wednesday: Wednesday,
    pub thursday: Thursday,
    pub friday: Friday,
    pub saturday: Saturday,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Sunday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Monday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tuesday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Wednesday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Thursday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Friday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Saturday {
    pub closed: OptionWrapper<bool>,
    pub open: OptionWrapper<i64>,
    pub close: OptionWrapper<i64>,
}

// [ SERVICES ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: OptionWrapper<String>,
    pub name: OptionWrapper<String>,
    pub alias: OptionWrapper<String>,
    pub building_id: OptionWrapper<String>,
    pub description: OptionWrapper<String>,
    pub campus: OptionWrapper<String>,
    pub address: OptionWrapper<String>,
    pub image: OptionWrapper<String>,
    pub coordinates: Coordinates,
    pub tags: OptionWrapper<String>,
    pub attributes: Vec<String>,
    pub last_updated: OptionWrapper<String>,
}

// [ BUILDINGS ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: OptionWrapper<String>,
    pub code: OptionWrapper<String>,
    pub tags: OptionWrapper<String>,
    pub name: OptionWrapper<String>,
    pub short_name: OptionWrapper<String>,
    pub address: BuildingAddress,
    pub coordinates: Coordinates,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BuildingAddress {
    pub street: OptionWrapper<String>,
    pub city: OptionWrapper<String>,
    pub province: OptionWrapper<String>,
    pub country: OptionWrapper<String>,
    pub postal: OptionWrapper<String>,
}

// [ PARKING ]

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Parking {
    pub id: OptionWrapper<String>,
    pub name: OptionWrapper<String>,
    pub alias: OptionWrapper<String>,
    pub building_id: OptionWrapper<String>,
    pub description: OptionWrapper<String>,
    pub campus: OptionWrapper<String>,
    pub address: OptionWrapper<String>,
    pub coordinates: Coordinates,
    pub last_updated: OptionWrapper<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: OptionWrapper<f64>,
    pub longitude: OptionWrapper<f64>,
}
