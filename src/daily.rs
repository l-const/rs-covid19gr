//! Daily recorded events

use serde::{Deserialize, Serialize};

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
    pub intensive_care: u32,
    pub date: String,
}

// HTTP GET /total-tests
// Test performed
/// Number of tests performed reported in Greece as timseries
#[derive(Debug, Deserialize, Serialize)]
pub struct TestSeries {
    pub cases: Vec<Tests>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tests {
    pub tests: u32,
    // check actual json-field : rapid-tests
    #[serde(rename = "rapid-tests")]
    pub rapid_tests: u32,
    pub date: String,
}

// HTTP GET /age-distribution-history
/// Age distribution of cases timeline
#[derive(Debug, Deserialize, Serialize)]
pub struct AgeDistributionSeries {
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
    pub cases: Option<String>,
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
    pub daydiff: u32,
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
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser_school_status() {
        let str_json: &str = r#"
            {
                "schools_status": [
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
    fn test_desser_vaccin_per_region_history() {
        let str_json: &str = r#"
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
        let vaccine_series_hist: Result<VaccineSeries, _> = serde_json::from_str(str_json);
        println!("{:?}", &vaccine_series_hist);
        assert!(vaccine_series_hist.is_ok());
    }

    #[test]
    fn test_females_cases_history() {
        let STR_JSON: &str = r#"
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
    fn test_males_cases_history() {
        let STR_JSON: &str = r#"
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
    fn test_confirmed() {
        let STR_JSON: &str = r#"
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
    fn test_recovered() {
        let STR_JSON: &str = r#"
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
    fn test_deaths() {
        let STR_JSON: &str = r#"
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
    fn test_active() {
        let STR_JSON: &str = r#"
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
}
