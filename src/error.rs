
use std::fmt;  
use std::error::Error as StdError;

#[derive(Debug)]
pub enum PlaneError {
    TooFewPoints,
    PointsNotProvided
}

// impl<'a> From<PlaneError> for &'a str {
//     fn from(e: PlaneError) -> &'a str {
//         match e {
//             PlaneError::TooFewPoints => "too few points provided. set more then one point",
//             PlaneError::PointsNotProvided => "plane has empty points field. set points using with_points(<points>)"
//         }
//     }
// }

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

// impl<'a> From<PointError> for &'a str {
//     fn from(e: PointError) -> Self {
//         match e {
//             PointError::Empty => "vec of points is empty or invalid"
//         }
//     }
// }

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