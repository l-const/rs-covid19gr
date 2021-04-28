//! Per hospital data

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
    pub home_restriction_current: u32,
    pub hospital_name: String,
    pub hospitalized_current: String,
    pub hospitalized_negative: String,
    pub hospitalized_pending_result: String,
    pub hospitalized_positive: String,
    pub new_recoveries: u32,
    pub new_samples: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalSlice {
    pub hospitalized_ICU_current: u32,
    pub total_deaths: u32,
    pub total_samples: u32,
    pub total_samples_negative: u32,
    pub total_Samples_positive: u32 
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_des_west_mac(){

    }
}


