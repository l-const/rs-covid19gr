//! Daily recorded events

use serde::Deserialize;

// HTTP GET /confirmed
// Confirmed cases  
/// Confirmed and fatal cases reported in Greece as timeseries
#[derive(Debug, Deserialize)]
pub struct ConfirmedSeries {
    cases: Vec<Confirmed>,
}
#[derive(Debug, Deserialize)]
pub struct Confirmed {
    confirmed: u32,
    date: String,
}

// HTTP GET /recovered
// Recovered cases
/// Recovered cases reported in Greece as timeseries. 
#[derive(Debug, Deserialize)]
pub struct RecoveredSeries {
    cases: Vec<Recovered>,
}

#[derive(Debug, Deserialize)]
pub struct Recovered {
    recovered: u32,
    date: String,
}

// HTTP GET /deaths
// Deaths
/// Deaths reported in Greece as timeseries.
#[derive(Debug, Deserialize)]
pub struct DeathSeries {
    cases: Vec<Deaths>,
}

#[derive(Debug, Deserialize)]
pub struct Deaths {
    deaths: u32,
    date: String,
}

// HTTP GET /active
// Active cases
/// Active cases reported in Greece as timeseries
#[derive(Debug, Deserialize)]
pub struct ActiveSeries {
    cases: Vec<Active>,
}


#[derive(Debug, Deserialize)]
pub struct Active {
    active: u32,
    date: String,
}

// HTTP GET /intensive-care
// Patients in intensive care
/// Patients in intensive care units reported in Greece as timseries
#[derive(Debug, Deserialize)]
pub struct IntensiveSeries {
    cases: Vec<Intensive>,
}

#[derive(Debug, Deserialize)]
pub struct Intensive {
    intensive_care: u32,
    date: String,
}

// HTTP GET /total-tests
// Test performed
/// Number of tests performed reported in Greece as timseries
#[derive(Debug, Deserialize)]
pub struct TestSeries {
    cases: Vec<Tests>,
}

#[derive(Debug, Deserialize)]
pub struct Tests {
    tests: u32,
    // check actual json-field : rapid-tests
    #[serde(rename = "rapid-tests")]
    rapid_tests: u32,
    date: String,
}

// HTTP GET /age-distribution-history
/// Age distribution of cases timeline
#[derive(Debug, Deserialize)]
pub struct AgeDistributionSeries {
    age_distribution: Vec<AgeDistributionSlice>,
}

#[derive(Debug, Deserialize)]
pub struct AgeDistributionSlice {
    date: String,
    cases: AgeSlice,
    critical: AgeSlice,
    deaths: AgeSlice,
}

#[derive(Debug, Deserialize)]
pub struct AgeSlice {
    #[serde(rename = "0-17")]
    age_group1: u32,
    #[serde(rename = "18-39")]
    age_group2: u32,
    #[serde(rename = "40-64")]
    age_group3: u32,
    #[serde(rename = "65+")]
    age_group4: u32,

}

/// Number of confirmed cases per region in Greece as timeseries
#[derive(Debug, Deserialize)]
pub struct RegionsHistorySeries {
    regions_history : Vec<Regions>
}


#[derive(Debug, Deserialize)]
pub struct Regions {
    date : String,
    regions:  Vec<RegionSlice>
}



#[derive(Debug, Deserialize)]
pub struct RegionSlice {
    area_en : String,
    area_gr : String,
    cases : Option<String>,
    geo_department_en : String,
    geo_department_gr : String,
    last_updated_at: String,
    latitude: f64,
    longtitude: f64,
    region_en: String,
    region_gr: String, 
}



#[cfg(test)]
mod tests {

    use super::*;
    
    

}
