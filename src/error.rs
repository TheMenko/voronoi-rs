
#[derive(Debug)]
pub enum PlaneError {
    TooFewPoints,
    PointsNotProvided
}

impl<'a> From<PlaneError> for &'a str {
    fn from(e: PlaneError) -> &'a str {
        match e {
            PlaneError::TooFewPoints => "too few points provided. set more then one point",
            PlaneError::PointsNotProvided => "plane has empty points field. set points using with_points(<points>)"
        }
    }
}