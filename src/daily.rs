//! Daily recorded events

use crate::build_request;
use serde::{Deserialize, Serialize};
use crate::macros;


// HTTP GET /all
/// All the information about the cases
#[derive(Debug, Deserialize, Serialize)]
pub struct AllSeries {
    cases: Vec<AllSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllSlice {
    confirmed: u32,
    deaths: u32,
    date: String,
}

// HTTP GET /confirmed
// Confirmed cases
/// Confirmed and fatal cases reported in Greece as timeseries
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfirmedSeries {
    pub cases: Vec<Confirmed>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Confirmed {
    pub confirmed: u32,
    pub date: String,
}

// HTTP GET /recovered
// Recovered cases
/// Recovered cases reported in Greece as timeseries.
#[derive(Debug, Deserialize, Serialize)]
pub struct RecoveredSeries {
    pub cases: Vec<Recovered>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recovered {
    pub recovered: u32,
    pub date: String,
}

// HTTP GET /deaths
// Deaths
/// Deaths reported in Greece as timeseries.
#[derive(Debug, Deserialize, Serialize)]
pub struct DeathSeries {
    pub cases: Vec<Deaths>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deaths {
    pub deaths: u32,
    pub date: String,
}

// HTTP GET /active
// Active cases
/// Active cases reported in Greece as timeseries
#[derive(Debug, Deserialize, Serialize)]
pub struct ActiveSeries {
    pub cases: Vec<Active>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Active {
    pub active: u32,
    pub date: String,
}

// HTTP GET /intensive-care
// Patients in intensive care
/// Patients in intensive care units reported in Greece as timseries
#[derive(Debug, Deserialize, Serialize)]
pub struct IntensiveSeries {
    pub cases: Vec<Intensive>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Intensive {
    pub intensive_care: Option<u32>,
    pub date: String,
}

// HTTP GET /total-tests
// Test performed
/// Number of tests performed reported in Greece as timseries
#[derive(Debug, Deserialize, Serialize)]
pub struct TestSeries {
    pub total_tests: Vec<Tests>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tests {
    pub tests: Option<u32>,
    #[serde(rename = "rapid-tests")]
    pub rapid_tests: u32,
    pub date: String,
}

// HTTP GET /age-distribution-history
/// Age distribution of cases timeline
#[derive(Debug, Deserialize, Serialize)]
pub struct AgeDistributionSeries {
    #[serde(rename = "age-distribution")]
    pub age_distribution: Vec<AgeDistributionSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgeDistributionSlice {
    pub date: String,
    pub cases: AgeSlice,
    pub critical: AgeSlice,
    pub deaths: AgeSlice,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgeSlice {
    #[serde(rename = "0-17")]
    pub age_group_0_17: u32,
    #[serde(rename = "18-39")]
    pub age_group_18_39: u32,
    #[serde(rename = "40-64")]
    pub age_group_40_64: u32,
    #[serde(rename = "65+")]
    pub age_group4_65: u32,
}

/// Number of confirmed cases per region in Greece as timeseries
#[derive(Debug, Deserialize, Serialize)]
pub struct RegionsHistorySeries {
    #[serde(rename = "regions-history")]
    pub regions_history: Vec<Regions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Regions {
    pub date: String,
    pub regions: Vec<RegionSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegionSlice {
    pub area_en: String,
    pub area_gr: String,
    pub cases: Option<u32>,
    pub geo_department_en: String,
    pub geo_department_gr: String,
    pub last_updated_at: String,
    pub latitude: f64,
    pub longtitude: f64,
    pub region_en: String,
    pub region_gr: String,
}

// HTTP GET /male-cases-history
/// Age distribution of male cases timeline
#[derive(Debug, Deserialize, Serialize)]
pub struct MaleCasesHistory {
    #[serde(rename = "male-cases")]
    pub male_cases: Vec<AgeDistributionSlice>,
}

// HTTP GET /female-cases-history
/// Age distribution of male cases timeline
#[derive(Debug, Deserialize, Serialize)]
pub struct FemaleCasesHistory {
    #[serde(rename = "female-cases")]
    pub female_cases: Vec<AgeDistributionSlice>,
}

// HTTP GET /vaccinations-per-region-history
/// Vaccinations per region timeline
#[derive(Debug, Deserialize, Serialize)]
pub struct VaccineSeries {
    #[serde(rename = "vaccinations-history")]
    pub vaccinations_history: Vec<VaccineSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaccineSlice {
    pub area_gr: String,
    pub area_en: String,
    pub dailydose1: u32,
    pub dailydose2: u32,
    pub daydiff: i32,
    pub daytotal: u32,
    pub referencedate: String,
    pub totaldistinctpersons: u32,
    pub totaldose1: u32,
    pub totaldose2: u32,
    pub totalvaccinations: u32,
}

// HTPP GET /school-status
/// School operation
#[derive(Debug, Deserialize, Serialize)]
pub struct SchoolStatusSeries {
    #[serde(rename = "schools-status")]
    pub schools_status: Vec<SchoolSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchoolSlice {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Area")]
    pub area: String,
    #[serde(rename = "DateFrom")]
    pub date_from: String,
    #[serde(rename = "DateTo")]
    pub date_to: String,
    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    #[serde(rename = "MunicipalUnit")]
    pub municipal_unit: String,
    #[serde(rename = "Municipality")]
    pub municipality: String,
    #[serde(rename = "Remarks")]
    pub remarks: String,
    #[serde(rename = "SchoolKind")]
    pub school_kind: String,
    #[serde(rename = "UnitName")]
    pub unit_name: String,
}

pub fn get_all_series_data() -> AllSeries {
    let json_resp = build_request("all");
    let all_series = serde_json::from_str(&json_resp).unwrap();
    all_series
}

pub fn get_confirmed_series_data() -> ConfirmedSeries {
    let json_resp = build_request("confirmed");
    let confirmed_series = serde_json::from_str(&json_resp).unwrap();
    confirmed_series
}

pub fn get_recovered_series_data() -> RecoveredSeries {
    let json_resp = build_request("recovered");
    let recovered_series = serde_json::from_str(&json_resp).unwrap();
    recovered_series
}

pub fn get_deaths_series_data() -> DeathSeries {
    let json_resp = build_request("deaths");
    let deaths_series = serde_json::from_str(&json_resp).unwrap();
    deaths_series
}

pub fn get_active_series_data() -> ActiveSeries {
    let json_resp = build_request("active");
    let active_series = serde_json::from_str(&json_resp).unwrap();
    active_series
}

pub fn get_intensive_care_series_data() -> IntensiveSeries {
    let json_resp = build_request("intensive-care");
    let intensive_series = serde_json::from_str(&json_resp).unwrap();
    intensive_series
}

pub fn get_total_tests_series_data() -> TestSeries {
    let json_resp = build_request("total-tests");
    let test_series = serde_json::from_str(&json_resp).unwrap();
    test_series
}

pub fn get_age_dist_series_data() -> AgeDistributionSeries {
    let json_resp = build_request("age-distribution-history");
    let age_dist_series = serde_json::from_str(&json_resp).unwrap();
    age_dist_series
}

pub fn get_regions_history_series_data() -> RegionsHistorySeries {
    let json_resp = build_request("regions-history");
    let region_hist_series = serde_json::from_str(&json_resp).unwrap();
    region_hist_series
}

pub fn get_male_cases_series_data() -> MaleCasesHistory {
    let json_resp = build_request("male-cases-history");
    let male_cases = serde_json::from_str(&json_resp).unwrap();
    male_cases
}

pub fn get_female_cases_series_data() -> FemaleCasesHistory {
    let json_resp = build_request("female-cases-history");
    let female_cases = serde_json::from_str(&json_resp).unwrap();
    female_cases
}

pub fn get_vaccin_per_region_series_data() -> VaccineSeries {
    let json_resp = build_request("vaccinations-per-region-history");
    let vaccine_series = serde_json::from_str(&json_resp).unwrap();
    vaccine_series
}

pub fn get_school_status_series_data() -> SchoolStatusSeries {
    let json_resp = build_request("schools-status");
    let school_series = serde_json::from_str(&json_resp).unwrap();
    school_series
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser_school_status() {
        let str_json: &str = r#"
            {
                "schools-status": [
                    {
                    "UnitName": "string",
                    "SchoolKind": "string",
                    "Municipality": "string",
                    "MunicipalUnit": "string",
                    "Area": "string",
                    "Address": "string",
                    "Longitude": 0,
                    "Latitude": 0,
                    "DateFrom": "2021-04-28",
                    "DateTo": "2021-04-28",
                    "Remarks": "string"
                    }
                ]
            }
        "#;
        let school_data: Result<SchoolStatusSeries, _> = serde_json::from_str(str_json);
        println!("{:?}", &school_data);
        assert!(school_data.is_ok());
    }

    #[test]
    fn test_deser_vaccin_per_region_history() {
        const STR_JSON: &str = r#"
                        {
                "vaccinations-history": [
                    {
                        "area_gr": "string",
                        "area_en": "string",
                        "dailydose1": 0,
                        "dailydose2": 0,
                        "daydiff": 0,
                        "daytotal": 0,
                        "referencedate": "2021-04-28",
                        "totaldistinctpersons": 0,
                        "totaldose1": 0,
                        "totaldose2": 0,
                        "totalvaccinations": 0
                    }
                ]
             }
        "#;
        let vaccine_series_hist: Result<VaccineSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &vaccine_series_hist);
        assert!(vaccine_series_hist.is_ok());
    }

    #[test]
    fn test_deser_region_history() {
        const STR_JSON: &str = r#"
     
        {
            "regions-history": [
              {
                "date": "2021-04-29",
                "regions": [
                  {
                    "area_gr": "string",
                    "area_en": "string",
                    "region_gr": "string",
                    "region_en": "string",
                    "geo_department_gr": "string",
                    "geo_department_en": "string",
                    "last_updated_at": "2021-04-29",
                    "longtitude": 0,
                    "latitude": 0,
                    "population": 0,
                    "cases": 0
                  }
                ]
              }
            ]
          }
        "#;
        let region_series_hist: Result<RegionsHistorySeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &region_series_hist);
        assert!(region_series_hist.is_ok());
    }
    #[test]
    fn test_deser_females_cases_history() {
        const STR_JSON: &str = r#"
            {
                "female-cases": [
                    {
                    "date": "2021-04-28",
                    "cases": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    },
                    "critical": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    },
                    "deaths": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    }
                    }
                ]
            }
        "#;
        let females_series_hist: Result<FemaleCasesHistory, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &females_series_hist);
        assert!(females_series_hist.is_ok());
    }

    #[test]
    fn test_deser_males_cases_history() {
        const STR_JSON: &str = r#"
            {
                "male-cases": [
                    {
                    "date": "2021-04-28",
                    "cases": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    },
                    "critical": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    },
                    "deaths": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    }
                    }
                ]
            }
        "#;
        let males_series_hist: Result<MaleCasesHistory, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &males_series_hist);
        assert!(males_series_hist.is_ok());
    }

    #[test]
    fn test_deser_age_dist_history() {
        const STR_JSON: &str = r#"
            {
                "age-distribution": [
                    {
                    "date": "2021-04-28",
                    "cases": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    },
                    "critical": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    },
                    "deaths": {
                        "0-17": 0,
                        "18-39": 0,
                        "40-64": 0,
                        "65+": 0
                    }
                    }
                ]
            }
        "#;
        let age_dist_series_hist: Result<AgeDistributionSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &age_dist_series_hist);
        assert!(age_dist_series_hist.is_ok());
    }

    #[test]
    fn test_deser_confirmed() {
        const STR_JSON: &str = r#"
          {
            "cases": [
                {
                "confirmed": 0,
                "date": "2021-04-28"
                }
            ]
           }
        "#;
        let confirmed: Result<ConfirmedSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &confirmed);
        assert!(confirmed.is_ok());
    }

    #[test]
    fn test_deser_recovered() {
        const STR_JSON: &str = r#"
          {
            "cases": [
                {
                "recovered": 0,
                "date": "2021-04-28"
                }
            ]
           }
        "#;
        let recovered: Result<RecoveredSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &recovered);
        assert!(recovered.is_ok());
    }

    #[test]
    fn test_desser_deaths() {
        const STR_JSON: &str = r#"
          {
            "cases": [
                {
                "deaths": 0,
                "date": "2021-04-28"
                }
            ]
           }
        "#;
        let deaths: Result<DeathSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &deaths);
        assert!(deaths.is_ok());
    }

    #[test]
    fn test_deser_active() {
        const STR_JSON: &str = r#"
          {
            "cases": [
                {
                "active": 0,
                "date": "2021-04-28"
                }
            ]
           }
        "#;
        let active: Result<ActiveSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &active);
        assert!(active.is_ok());
    }

    #[test]
    fn test_deser_all() {
        const STR_JSON: &str = r#"
          {
            "cases": [
                {
                "confirmed": 0,
                "deaths": 0,
                "date": "2021-04-29"
                }
            ]
           }
        "#;
        let all: Result<AllSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &all);
        assert!(all.is_ok());
    }

    #[test]
    fn test_desser_intensive_care() {
        const STR_JSON: &str = r#"
          {
            "cases": [
                {
                "intensive_care": 0,
                "date": "2021-04-29"
                }
            ]
           }
        "#;
        let intensive: Result<IntensiveSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &intensive);
        assert!(intensive.is_ok());
    }

    #[test]
    fn test_deser_total_tests() {
        const STR_JSON: &str = r#"
          {
            "total_tests": [
                {
                "tests": 0,
                "rapid-tests" : 0,
                "date": "2021-04-29"
                }
            ]
           }
        "#;
        let tests: Result<TestSeries, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", &tests);
        assert!(tests.is_ok());
    }

    #[test]
    fn test_get_all_data() {
        let res = get_all_series_data();
    }

    #[test]
    fn test_get_confirmed_data() {
        let res = get_confirmed_series_data();
    }

    #[test]
    fn test_get_recovered_data() {
        let res = get_recovered_series_data();
    }

    #[test]
    fn test_get_deaths_series_data() {
        let res = get_deaths_series_data();
    }

    #[test]
    fn test_get_active_series_data() {
        let res = get_active_series_data();
    }

    #[test]
    fn test_get_intensive_care_data() {
        let res = get_intensive_care_series_data();
    }

    #[test]
    fn test_get_total_tests_data() {
        let res = get_total_tests_series_data();
    }

    #[test]
    fn test_get_age_dist_series_data() {
        let res = get_age_dist_series_data();
    }

    #[test]
    fn test_get_regions_history_data() {
        let res = get_regions_history_series_data();
    }

    #[test]
    fn test_get_males_cases_series_data() {
        let res = get_male_cases_series_data();
    }

    #[test]
    fn test_get_females_cases_series_data() {
        let res = get_female_cases_series_data();
    }

    #[test]
    fn test_vaccin_history_data() {
        let res = get_vaccin_per_region_series_data();
    }

    #[test]
    fn test_get_school_status_data() {
        let res = get_school_status_series_data();
    }
}
