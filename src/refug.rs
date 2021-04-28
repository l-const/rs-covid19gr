//! Per refugee camp data

use serde::{Deserialize, Serialize};

// HTTP GET /regugee-camps
/// Recorded cases in Refugee-camps.
#[derive(Debug, Deserialize, Serialize)]
pub struct RefCamps {
   #[serde(rename = "refugee-camps")]
   pub refugee_camps: Vec<RefCamp>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefCamp {
    pub area_type_en : String, 
    pub area_type_gr:  String,
    pub capacity: Option<u32>,
    pub current_hosts: Option<u32>,
    pub description: String,
    pub last_update: String,
    pub latitude : f64,
    pub longtitude: f64,
    pub name_en: String,
    pub name_gr: String,
    pub recorded_events : Vec<RecordedEvent>,
    pub region_en: String,
    pub region_gr: String,
    pub total_confirmed_cases: u32,
    pub total_samples: u32
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RecordedEvent {
   pub case_detection_week :  Option<String>, 
   pub confirmed_cases: Option<u32>,
   pub quarantine_duration_days : Option<u32>,
   pub samples: Option<u32>
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_des_refugee(){

    }
}