
use std::fmt;  
use std::error::Error as StdError;

#[derive(Debug)]
pub enum PlaneError {
    TooFewPoints,
    PointsNotProvided
}

impl From<PlaneError> for String {
    fn from(e: PlaneError) -> String {
        match e {
            PlaneError::TooFewPoints => "too few points provided. set more then one point".to_string(),
            PlaneError::PointsNotProvided => "plane has empty points field. set points using with_points(<points>)".to_string()
        }
    }
}

impl fmt::Display for PlaneError {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlaneError::TooFewPoints => f.write_str("too few points provided. set more then one point"),
            PlaneError::PointsNotProvided => f.write_str("plane has empty points field. set points using with_points(<points>)"),
        }
    }
}

impl StdError for PlaneError {  
    fn description(&self) -> &str {
        match *self {
            PlaneError::TooFewPoints => "too few points provided. set more then one point",
            PlaneError::PointsNotProvided => "plane has empty points field. set points using with_points(<points>)",
        }
    }
}

#[derive(Debug)]
pub enum PointError {
    Empty
}

impl From<PointError> for String {
    fn from(e: PointError) -> String {
        match e {
            PointError::Empty => "vec of points is empty or invalid".to_string(),
        }
    }
}

impl fmt::Display for PointError {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PointError::Empty => f.write_str("vec of points is empty or invalid"),
        }
    }
}

impl StdError for PointError {  
    fn description(&self) -> &str {
        match *self {
            PointError::Empty => "vec of points is empty or invalid",
        }
    }
}