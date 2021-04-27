//! Daily recorded events

/// HTTP GET /confirmed
/// Confirmed cases
/// Get all the confirmed and fatal cases reported in Greece as timeseries
pub struct ConfirmedSeries {
    cases: Vec<Confirmed>,
}

pub struct Confirmed {
    confirmed: u32,
    date: String,
}

/// HTTP GET /recovered
/// Recovered cases
/// Get all the recovered cases reported in Greece as timeseries
/// Data scarce availability
pub struct RecoveredSeries {
    cases: Vec<Recovered>,
}

pub struct Recovered {
    recovered: u32,
    date: String,
}

/// HTTP GET /deaths
/// Deaths
/// Get all the deaths reported in Greece as timeseries
pub struct DeathSeries {
    cases: Vec<Deaths>,
}

pub struct Deaths {
    deaths: u32,
    date: String,
}

/// HTTP GET /active
/// Active cases
/// Get all the active cases reported in Greece as timeseries
/// Data scarce availability
pub struct ActiveSeries {
    cases: Vec<Active>,
}

pub struct Active {
    active: u32,
    date: String,
}

/// HTTP GET /intensive-care
/// Patients in intensive care
/// Get the number of patients in intensive care units reported in Greece as timseries
pub struct IntensiveSeries {
    cases: Vec<Intensive>,
}

pub struct Intensive {
    intensive_care: u32,
    date: String,
}

/// HTTP GET /total-tests
/// Test performed
/// Get the number of tests performed reported in Greece as timseries
pub struct TestSeries {
    cases: Vec<Tests>,
}

pub struct Tests {
    tests: u32,
    // check actual json-field : rapid-tests
    rapid_tests: u32,
    date: String,
}

/// HTTP GET /age-distribution-history
/// Age distribution of cases timeline
/// Get the timeline of the age distribution of the cases reported in Greece
pub struct AgeDistributionSeries {
    age_distribution: Vec<AgeDistributionSlice>,
}

pub struct AgeDistributionSlice {}
