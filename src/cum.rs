//! Cumulative data

// HTTP GET /total
/// Latest total cummulative data in Greece
pub struct Total {
    cases: TotalSlice,
}

pub struct TotalSlice {
    confirmed: u32,
    date: String,
    deaths: u32,
}

// HTTP GET /regions
/// Latest cumulative number of confirmed cases per region in Greece
pub struct Regions {
    regions: Vec<Region>,
}

pub struct Region {
    area_gr: String,
    area_en: String,
    region_gr: String,
    region_en: String,
    geo_department_gr: String,
    geo_department_en: String,
    last_updated_at: String,
    longtitude: f64,
    latitude: f64,
    population: u32,
    total_cases: u32,
    cases_per_100000_people: u32,
}
