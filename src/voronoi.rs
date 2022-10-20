#![allow(unused_variables)]

use crate::plane::Plane;
use crate::point::Point;
use crate::error::VoronoiError;

use rayon::prelude::*;

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
    
    fn recursive_calc(points: Vec<Point>) {
        
    
    }
    
    fn calc_for_edges(self, points: &Vec<Point>) -> Vec<(u8, Point)> {
        
        let calc_dist_for_edge = |edge: Point, points: &Vec<Point>| -> Point {
            Point::new(0., 0.)
        };
        
        //let p = points.clone();
        let corners = [
            (Point::new(0., 0.), 1u8), 
            (Point::new(self.plane.width as f64, 0.), 2u8), 
            (Point::new(self.plane.width as f64, self.plane.height as f64), 3u8), 
            (Point::new(self.plane.height as f64, 0.), 4u8)];
            
        let mut closest_points: Vec<(u8, Point)> = corners.par_iter().map(|i| (i.1, calc_dist_for_edge(i.0, points))).collect();
        
        closest_points.sort_by(|k,p| k.0.partial_cmp(&p.0).unwrap());
        closest_points
    }
    
    pub fn build(&mut self) -> Result<Voronoi> {
        
        // sort points
        self.plane.sort_by_apsci();
        // split on two so we can later merge them when voronoi is calculated for both
        let (left, right) = match self.plane.get_subsets_vertical() {
            Ok(lr) => lr,
            Err(e) => { return Err(VoronoiError::Error(String::from("varanoi error: "),e.into())); }
        };
        
        
        
        Ok(Voronoi{})
    }
}