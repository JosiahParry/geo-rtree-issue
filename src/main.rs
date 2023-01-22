
use geo::{BoundingRect, Intersects};
use geo_types::{Polygon};
use rstar::{RTree, AABB};

use std::fs::File;
use std::io::{BufReader, BufRead};
use wkt::{TryFromWkt};

use std::time::{Instant};


mod geortree;
use crate::geortree::*;



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
    let mut r_tree: RTree<TreeNode> = RTree::new();

    for (index, geom) in all_polys.clone().into_iter().enumerate() {

        let env = NodeEnvelope::from(geom);
        let node = TreeNode {
            index,
            envelope: env
        };
        r_tree.insert(node);

    }


    
    // find candidates just once using R Tree
    let start = Instant::now();
    let rect = all_polys[0].clone().bounding_rect().unwrap();
    let bbox = [[rect.min().x, rect.min().y],
            [rect.max().x, rect.max().y]];
    
    let intersect_candidates = r_tree.
        locate_in_envelope_intersecting(&AABB::from_corners(bbox[0], bbox[1]));
    let indexes: Vec<usize> = intersect_candidates.map(|node| node.index).collect();
    let end = start.elapsed();
    println!("Candidates found in {:?}\n", end);
    println!("Possible intersections: {:?}\n", indexes);


    let geom = all_polys[0].clone();
    // find the candidates and then check if actually intersecting
    let start = Instant::now();
    //let index = find_candidate_indexes(r_tree.clone(), all_polys[0].clone());

    let mut true_hits = Vec::new();

    for cand_index in indexes.clone() {    
        let hit = geom.intersects(&all_polys[cand_index]);
        if hit {
            true_hits.push(cand_index);
        }
    }
    let end = start.elapsed();
    println!("Actual intersections using candidates: {:?}", true_hits);
    println!("  found in {:?}\n", end);



    // find the candidates and then check if actually intersecting
    let start = Instant::now();
    //let index = find_candidate_indexes(r_tree.clone(), all_polys[0].clone());

    let mut true_hits = Vec::new();
    for cand_index in 0..all_polys.len() {    

        let hit = geom.intersects(&all_polys[cand_index]);

        if hit {
            true_hits.push(cand_index);
        }
    }
    let end = start.elapsed();

    println!("Actual intersections iterating over everything: {:?}", true_hits);
    println!("  found in {:?}", end);
    

}


