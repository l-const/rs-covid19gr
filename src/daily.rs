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
    pub age_group1: u32,
    #[serde(rename = "18-39")]
    pub age_group2: u32,
    #[serde(rename = "40-64")]
    pub age_group3: u32,
    #[serde(rename = "65+")]
    pub age_group4: u32,
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
pub struct SchoolStatusSeries {
    pub school_status: Vec<SchoolSlice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchoolSlice {
    pub Address: String,
    pub Area: String,
    pub DateFrom: String,
    pub DateTo: String,
    pub Latitude: f64,
    pub Longtitude: f64,
    pub MunicipalUnit: u32,
    pub Municipality: String,
    pub Remarks: String,
    pub SchoolKind: String,
    pub UnitName: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Ser() {}
}
