//! Per refugee camp data

use serde::{Deserialize, Serialize};

use crate::build_request;
use crate::BASE_URL;

// HTTP GET /regugee-camps
/// Recorded cases in Refugee-camps.
#[derive(Debug, Deserialize, Serialize)]
pub struct RefCamps {
    #[serde(rename = "refugee-camps")]
    pub refugee_camps: Vec<RefCamp>,
}

impl IntoIterator for RefCamps {
    type Item = RefCamp;
    type IntoIter = std::vec::IntoIter<RefCamp>;

    fn into_iter(self) -> Self::IntoIter {
        self.refugee_camps.into_iter()
    }
}

/// Slice Iterator for RefCamp.
pub type IterRefCamp<'iter> = core::slice::Iter<'iter, RefCamp>;
/// Mutable slice Iterator for RefCamp.
pub type IterMutRefCamp<'iter_mut> = core::slice::IterMut<'iter_mut, RefCamp>;

impl RefCamps {
    pub fn iter(&self) -> IterRefCamp {
        self.refugee_camps.iter()
    }

    pub fn iter_mut(&mut self) -> IterMutRefCamp {
        self.refugee_camps.iter_mut()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefCamp {
    pub area_type_en: String,
    pub area_type_gr: String,
    pub capacity: Option<u32>,
    pub current_hosts: Option<u32>,
    pub description: String,
    pub last_update: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub name_en: String,
    pub name_gr: String,
    pub recorded_events: Vec<RecordedEvent>,
    pub region_en: String,
    pub region_gr: String,
    pub total_confirmed_cases: u32,
    pub total_confirmed_samples: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordedEvent {
    pub case_detection_week: Option<String>,
    pub confirmed_cases: Option<u32>,
    pub quarantine_duration_days: Option<u32>,
    pub samples: Option<u32>,
}

pub fn get_refugee_camp_data() -> RefCamps {
    let json_resp = build_request("refugee-camps");
    let ref_camps = serde_json::from_str(&json_resp).unwrap();
    ref_camps
}

#[cfg(test)]
mod tests {

    use super::*;

    const str_json: &str = r#"{
              "refugee-camps":
              [
                               {
                                "area_type_en": "string",
                                "area_type_gr": "string",
                                "capacity": 0,
                                "current_hosts": 0,
                                "description": "string",
                                "last_update": "2021-04-28",
                                "latitude": 0,
                                "longtitude": 0,
                                "name_en": "string",
                                "name_gr": "string",
                                "recorded_events": [
                                    {
                                    "case_detection_week": "string",
                                    "confirmed_cases": 0,
                                    "quarantine_duration_days": 0,
                                    "samples": 0
                                    }
                                ],
                                "region_en": "string",
                                "region_gr": "string",
                                "total_confirmed_cases": 0,
                                "total_confirmed_samples": 0
                                }
                 ]
                            }"#;

    #[test]
    fn test_deserilize_refugee() {
        let ref_camps: Result<RefCamps, _> = serde_json::from_str(str_json);
        assert!(ref_camps.is_ok());
        //println!("RefCamps: {:?}", &ref_camps);
    }

    #[test]
    fn test_get_refugee_camp_data() {}

    #[test]
    fn test_into_iterator() {
        let ref_camps: RefCamps = serde_json::from_str(str_json).unwrap();
        ref_camps
            .into_iter()
            .for_each(|ref_camp| println!("{:?}", ref_camp));
    }

    #[test]
    fn test_iter() {
        let ref_camps: RefCamps = serde_json::from_str(str_json).unwrap();
        ref_camps.iter().for_each(|r| println!("{:?}", r));
    }

    #[test]
    fn test_iter_mut() {
        let mut ref_camps: RefCamps = serde_json::from_str(str_json).unwrap();
        let mut iter_mut = ref_camps.iter_mut();
        let mut first = iter_mut.next();
        let mut second = iter_mut.next();
        println!("{:?} {:?}", first, second);
    }
}
