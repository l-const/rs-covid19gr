//! Per hospital data

use crate::{impl_into_iter, impl_iter_and_mut, macros};

use crate::build_request;
use serde::{Deserialize, Serialize};

// HTTP GET /western-macedonia
/// Hospital data from the Region of Western Macedonia
#[derive(Debug, Deserialize, Serialize)]
pub struct WestMacSeries {
    #[serde(rename = "western-macedonia")]
    pub western_macedonia: Vec<WestMacSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WestMacSlice {
    pub date: String,
    pub hospitals: Vec<HospitalSlice>,
    pub total: TotalSlice,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HospitalSlice {
    pub home_restriction_current: f32,
    pub hospital_name: String,
    pub hospitalized_current: f32,
    pub hospitalized_negative: f32,
    pub hospitalized_pending_result: f32,
    pub hospitalized_positive: f32,
    pub new_recoveries: f32,
    pub new_samples: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalSlice {
    #[serde(rename = "hospitalized_ICU_current")]
    pub hospitalized_icu_current: f32,
    pub total_deaths: f32,
    pub total_samples: f32,
    pub total_samples_negative: f32,
    pub total_samples_positive: f32,
}

// HTTP GET /western-macedonia-deaths
/// Death records in Western Macedonia
#[derive(Debug, Deserialize, Serialize)]
pub struct WestMacDeathSeries {
    #[serde(rename = "western-macedonia-deaths")]
    pub western_macedonia_deaths: Vec<WestMacDeathSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WestMacDeathSlice {
    pub age: f32,
    pub date: String,
    pub permanent_residence_municipality_en: String,
    pub permanent_residence_municipality_gr: String,
    pub sex: String,
    pub underlying_diseases: String,
}

pub fn get_western_macedonia_data() -> WestMacSeries {
    let json_resp = build_request("western-macedonia");
    let west_mac_data = serde_json::from_str(&json_resp).unwrap();
    west_mac_data
}

pub fn get_western_macedonia_death_data() -> WestMacDeathSeries {
    let json_resp = build_request("western-macedonia-deaths");
    let west_mac_death_data = serde_json::from_str(&json_resp).unwrap();
    west_mac_death_data
}

// Macros impl
impl_into_iter!(WestMacSeries, WestMacSlice, western_macedonia);
impl_iter_and_mut!(WestMacSeries, WestMacSlice, western_macedonia);
impl_into_iter!(
    WestMacDeathSeries,
    WestMacDeathSlice,
    western_macedonia_deaths
);
impl_iter_and_mut!(
    WestMacDeathSeries,
    WestMacDeathSlice,
    western_macedonia_deaths
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_des_west_mac() {
        const STR_JSON: &str = r#"
                            {
                    "western-macedonia": [
                        {
                        "date": "2021-04-28",
                        "total": {
                            "hospitalized_ICU_current": 0,
                            "total_deaths": 0,
                            "total_samples": 0,
                            "total_samples_negative": 0,
                            "total_samples_positive": 0
                        },
                        "hospitals": [
                            {
                            "home_restriction_current": 0,
                            "hospital_name": "string",
                            "hospitalized_current": 0,
                            "hospitalized_negative": 0,
                            "hospitalized_pending_result": 0,
                            "hospitalized_positive": 0,
                            "new_recoveries": 0,
                            "new_samples": 0
                            }
                        ]
                        }
                    ]
                    }
        "#;
        let west_mac_data: Result<WestMacSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", west_mac_data);
        assert!(west_mac_data.is_ok());
    }

    #[test]
    fn test_des_west_mac_deaths() {
        const STR_JSON: &str = r#"
            {
                "western-macedonia-deaths": [
                    {
                    "age": 0,
                    "date": "2021-04-28",
                    "permanent_residence_municipality_en": "string",
                    "permanent_residence_municipality_gr": "string",
                    "sex": "string",
                    "underlying_diseases": "string"
                    }
                ]
             }
        "#;
        let west_mac_death_data: Result<WestMacDeathSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", west_mac_death_data);
        assert!(west_mac_death_data.is_ok());
    }

    #[test]
    fn test_get_west_mac() {
        let json_resp = get_western_macedonia_data();
    }

    #[test]
    fn test_get_west_mac_deaths() {
        let json_resp = get_western_macedonia_death_data();
    }
}
