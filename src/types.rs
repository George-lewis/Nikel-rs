use serde::{Deserialize, Serialize};

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