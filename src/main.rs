
use geo::{BoundingRect, Intersects};
use geo_types::{Polygon, Point, point};
use rstar::{RTree, AABB};

use std::fs::File;
use std::io::{BufReader, BufRead};
use wkt::{TryFromWkt};

use std::time::{Instant};



use rstar::primitives::GeomWithData;
//mod geortree;
//use crate::geortree::*;
mod Spatialndex;
use crate::Spatialndex::*;



fn to_point(arr: [f64;2]) -> Point{
    point! { x: arr[0], y: arr[1]}
}

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


    
    // find candidates just once using R Tree
    let start = Instant::now();
    let rect = all_polys[0].clone().bounding_rect().unwrap();
    let bbox = [[rect.min().x, rect.min().y],
            [rect.max().x, rect.max().y]];

//    rstar::Point
    
    let intersect_candidates = r_tree.
        locate_in_envelope_intersecting(
            &AABB::from_corners(to_point(bbox[0]), to_point(bbox[1]))
        );
    let indexes: Vec<usize> = intersect_candidates.
        map(|node| node.data).
        collect();
    let end = start.elapsed();
    println!("Candidates found in {:?}\n", end);
    println!("Possible intersections: {:?}\n", indexes);

    // // clone geom from the polygon vector
    // let geom = all_polys[0].clone();

    // // find the candidates and then check if actually intersecting
    // let start = Instant::now();


    // let mut true_hits = Vec::new();

    // for cand_index in indexes.clone() {    

    //     let start_int = Instant::now();
    //     let hit = geom.intersects(&all_polys[cand_index]);
    //     println!("Index {cand_index} took {:?}", start_int.elapsed());
    //     if hit {
    //         true_hits.push(cand_index);
    //     }
    // }
    // let end = start.elapsed();
    // println!("Actual intersections using candidates: {:?}", true_hits);
    // println!("  found in {:?}\n", end);



    // // check every polygon for intersection
    // let start = Instant::now();

    // let mut true_hits = Vec::new();
    // for cand_index in 0..all_polys.len() {    

    //     let hit = geom.intersects(&all_polys[cand_index]);

    //     if hit {
    //         true_hits.push(cand_index);
    //     }
    // }
    // let end = start.elapsed();

    // println!("Actual intersections iterating over everything: {:?}", true_hits);
    // println!("  found in {:?}", end);
    

}


