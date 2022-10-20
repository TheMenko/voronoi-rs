
use std::fmt;  
use std::error::Error as StdError;

#[derive(Debug)]
pub enum PlaneError {
    TooFewPoints,
    PointsNotProvided,
    SortedIncorrectly,
}

impl From<PlaneError> for String {
    fn from(e: PlaneError) -> String {
        match e {
            PlaneError::TooFewPoints => "too few points provided. set more then one point".to_string(),
            PlaneError::PointsNotProvided => "plane has empty points field. set points using with_points(<points>)".to_string(),
            PlaneError::SortedIncorrectly => "sorted incorrectly".to_string()
        }
    }
}

impl fmt::Display for PlaneError {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlaneError::TooFewPoints => f.write_str("too few points provided. set more then one point"),
            PlaneError::PointsNotProvided => f.write_str("plane has empty points field. set points using with_points(<points>)"),
            PlaneError::SortedIncorrectly => f.write_str("sorted incorrectly")
        }
    }
}

impl StdError for PlaneError {  
    fn description(&self) -> &str {
        match *self {
            PlaneError::TooFewPoints => "too few points provided. set more then one point",
            PlaneError::PointsNotProvided => "plane has empty points field. set points using with_points(<points>)",
            PlaneError::SortedIncorrectly => "sorted incorrectly"
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

#[derive(Debug)]
pub enum VoronoiError {
    Error(String, String)
}

impl From<VoronoiError> for String {
    fn from(e: VoronoiError) -> String {
        match e {
            VoronoiError::Error(message, other) => format!("{}. {}", message, other),
        }
    }
}

impl fmt::Display for VoronoiError {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VoronoiError::Error(message, other) => f.write_str(&format!("{}. {}", message, other)),
        }
    }
}