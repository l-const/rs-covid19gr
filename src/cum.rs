//! Cumulative data

use serde::{Deserialize, Serialize};

use crate::build_request;
use crate::macros;


// HTTP GET /total
/// Latest total cummulative data in Greece
#[derive(Debug, Deserialize, Serialize)]
pub struct Total {
    pub cases: TotalSlice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalSlice {
    pub confirmed: u32,
    pub date: String,
    pub deaths: u32,
}

// HTTP GET /regions
/// Latest cumulative number of confirmed cases per region in Greece
#[derive(Debug, Serialize, Deserialize)]
pub struct Regions {
    pub regions: Vec<Region>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub area_gr: String,
    pub area_en: String,
    pub region_gr: String,
    pub region_en: String,
    pub geo_department_gr: String,
    pub geo_department_en: String,
    pub last_updated_at: String,
    pub longtitude: f64,
    pub latitude: f64,
    pub population: u32,
    pub total_cases: u32,
    pub cases_per_100000_people: f32,
}

// HTTP GET /age-distribution
/// Age distribution of patients
#[derive(Debug, Serialize, Deserialize)]
pub struct AgeDistribution {
    pub age_distribution: AgeDistItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeDistItem {
    pub age_average: f32,
    pub average_death_age: f32,
    pub total_age_groups: TotalAgeRes,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalAgeRes {
    pub cases: AgeSlice,
    pub critical: AgeSlice,
    pub deaths: AgeSlice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeSlice {
    #[serde(rename = "0-17")]
    pub age_group1: u32,
    #[serde(rename = "18-39")]
    pub age_group2: u32,
    #[serde(rename = "40-64")]
    pub age_group3: u32,
    #[serde(rename = "65+")]
    pub age_group4: u32,
}

// HTTP GET /gender-distribution
/// Gender distribution of patients
#[derive(Debug, Serialize, Deserialize)]
pub struct GenderDistribution {
    pub gender_percentages: GenderDistItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenderDistItem {
    pub total_females_percentage: f32,
    pub total_males_percentage: f32,
    pub updated: String,
}

// HTTP /total-vaccinations
/// Total vaccinations
#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVaccinations {
    #[serde(rename = "total-vaccinations")]
    total_vaccinations: TotalVaccineSlice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVaccineSlice {
    totaldistinctpersons: u32,
    totalvaccinations: u32,
    updated: String,
}

// TODO : /total-vaccinations-per-region
// TODO : /gender-age-distribution
// TODO : /measures-timeline
// TODO : /risk-levels
// TODO : /measures-by-risk-level

/// Get the latest cumulative data in Greece
pub fn get_total_data() -> Total {
    let json_resp = build_request("total");
    let total_ser = serde_json::from_str(&json_resp).unwrap();
    total_ser
}

/// Get the latest cumulative number of confirmed cases per region in Greece
pub fn get_regions_data() -> Regions {
    let json_resp = build_request("regions");
    let regions_ser = serde_json::from_str(&json_resp).unwrap();
    regions_ser
}

/// Get the age distribution of the patients reported in Greece
pub fn get_age_dist_data() -> AgeDistribution {
    let json_resp = build_request("age-distribution");
    let age_dist_ser = serde_json::from_str(&json_resp).unwrap();
    age_dist_ser
}

/// Get the gender ratio of the patients reported in Greece as a percentage (%)
pub fn get_gender_dist_data() -> GenderDistribution {
    let json_resp = build_request("gender-distribution");
    let gender_dist_ser = serde_json::from_str(&json_resp).unwrap();
    gender_dist_ser
}

/// Get the total vaccinations in Greece
pub fn get_total_vaccin_data() -> TotalVaccinations {
    let json_resp = build_request("total-vaccinations");
    let total_vaccin_ser = serde_json::from_str(&json_resp).unwrap();
    total_vaccin_ser
}

mod tests {
    use super::*;

    #[test]
    fn test_des_total() {
        const STR_JSON: &str = r#"
        {
          "cases": {
                "confirmed": 0,
                "deaths": 0,
                "date": "2021-04-29"
           }
        }
        "#;
        let total: Result<Total, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", total);
        assert!(total.is_ok());
    }

    #[test]
    fn test_get_total_data() {
        let total_series = get_total_data();
    }

    #[test]
    fn test_des_regions() {
        const STR_JSON: &str = r#"
        {
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
                "total_cases": 0,
                "cases_per_100000_people": 0
              }
            ]
          }
        "#;
        let regions: Result<Regions, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", regions);
        assert!(regions.is_ok());
    }

    #[test]
    fn test_get_region_data() {
        let region_series = get_regions_data();
    }

    #[test]
    fn test_des_age_distribution() {
        const STR_JSON: &str = r#"
            {
                "age_distribution": {
                    "age_average": 0,
                    "average_death_age": 0,
                    "total_age_groups": {
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
                },
                    "updated": "2021-04-29"
                    
                }
            }
        "#;

        let age_dist: Result<AgeDistribution, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", age_dist);
        assert!(age_dist.is_ok());
    }

    #[test]
    fn test_get_age_dist_data() {
        let age_dist_series = get_age_dist_data();
    }

    #[test]
    fn test_des_gender_distribution() {
        const STR_JSON: &str = r#"
         {
            "gender_percentages" : {
                "total_females_percentage" : 0,
                "total_males_percentage": 0,
                "updated": "2021-04-29"
            }
         }
        "#;
        let gender_dist: Result<GenderDistribution, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", gender_dist);
        assert!(gender_dist.is_ok());
    }

    #[test]
    fn test_get_gender_dist_data() {
        let gender_dist_series = get_gender_dist_data();
    }

    #[test]
    fn test_des_total_vaccinations() {
        const STR_JSON: &str = r#"
                {
                    "total-vaccinations": {
                        
                            "totaldistinctpersons" : 0,
                            "totalvaccinations": 0,
                            "updated": "2021-04-29"
                        
                    }
                } 
        "#;
        let total_vacc: Result<TotalVaccinations, _> = serde_json::from_str(STR_JSON);
        println!("{:?}", total_vacc);
        assert!(total_vacc.is_ok());
    }

    #[test]
    fn test_get_total_vaccin_data() {
        let total_vaccin_series = get_total_vaccin_data();
    }
}
