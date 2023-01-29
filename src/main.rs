
mod erm;
use crate::erm::*;


//use geo::{BoundingRect, Intersects};
use geo_types::{Polygon};
use rstar::RTree;
use std::fs::File;
use std::io::{BufReader, BufRead};
use wkt::{TryFromWkt};

use rstar::primitives::GeomWithData;


fn main() {

    // read geometries from a text file
    let f = File::open("geoms.txt").expect("this shit to work"); 
    let f = BufReader::new(f);

    // creater a vector of polygons
    let mut all_polys: Vec<Polygon> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let ply: Polygon<f64> = Polygon::try_from_wkt_str(line.as_str()).unwrap();
        all_polys.push(ply);
    
    }

    // create the tree
    let mut r_tree = RTree::new();

    // insert into rtree with index as data
    for (index, geom) in all_polys.clone().into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }


    let papa = r_tree.root();


    let x = inner(papa);

    
}


