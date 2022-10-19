
use crate::plane::Plane;
use crate::point::Point;
use crate::error::VoronoiError;

type Result<T> = std::result::Result<T, VoronoiError>;

pub struct Voronoi {
    
}

pub struct VoronoiBuilder {
    plane: Plane
}

impl VoronoiBuilder {
    pub fn new(width: u32, height: u32, points: Vec<(f64, f64)>) -> Result<VoronoiBuilder> {
        
        let mut new_plane = Plane::new(width, height);
        match new_plane.with_points(match Point::from_vec(points) {
            Ok(p) => p,
            Err(e) => { return Err(VoronoiError::Error(String::from("failed at creating voronoi instance"),e.into())); }
        }) {
            Ok(_) => (),
            Err(e) => { return Err(VoronoiError::Error(String::from("failed at creating voronoi instance"),e.into())); }
        }
        
        Ok(Self {
            plane: new_plane
        })
    }
    
    pub fn build(&self) -> Result<Voronoi> {
        
        Ok(Voronoi{})
    }
}