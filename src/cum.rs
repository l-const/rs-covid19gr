//! Cumulative data

use serde::{Deserialize, Serialize};

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
    pub cases_per_100000_people: u32,
}

// HTTP GET /age-distribution
/// Age distribution of patients
#[derive(Debug, Serialize, Deserialize)]
pub struct AgeDistribution {
    pub age_distribution: AgeDistItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeDistItem {
    pub age_average: u32,
    pub average_death_age: u8,
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
    pub gender_percentages: AgeDistItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenderDistItem {
    pub total_females_percenage: f32,
    pub total_males_percentage: u32,
    pub updated: String,
}

// TODO : /total-vaccinations-per-region
// TODO : /gender-age-distribution
// TODO : /measures-timeline
// TODO : /risk-levels
// TODO : /measures-by-risk-level

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVaccinations {
    #[serde(rename = "total-vaccinations")]
    total_vaccinations: TotalVaccineSlice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalVaccineSlice {}

mod tests {
    use super::*;

    #[test]
    fn test_des_region() {}
}
