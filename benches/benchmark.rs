#[macro_use]
extern crate criterion;
extern crate geo;
extern crate spatialindex;

use crate::geo::{BoundingRect, Intersects, Polygon};
use criterion::Criterion;
use rstar::{RTree, AABB};
use spatialindex::{NodeEnvelope, TreeNode};
use std::io::BufRead;
use std::{fs::File, io::BufReader};
use wkt::TryFromWkt;

fn criterion_benchmark(c: &mut Criterion) {
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
    let geom = all_polys[0].clone();

    c.bench_function("Tree-assisted", |bencher| {
        bencher.iter(|| {
            // create the tree
            let mut r_tree: RTree<TreeNode> = RTree::new();

            for (index, geom) in all_polys.clone().into_iter().enumerate() {
                let env = NodeEnvelope::from(geom);
                let node = TreeNode {
                    index,
                    envelope: env,
                };
                r_tree.insert(node);
            }

            let rect = all_polys[0].clone().bounding_rect().unwrap();
            let bbox = [[rect.min().x, rect.min().y], [rect.max().x, rect.max().y]];

            let intersect_candidates =
                r_tree.locate_in_envelope_intersecting(&AABB::from_corners(bbox[0], bbox[1]));
            let indexes: Vec<usize> = intersect_candidates.map(|node| node.index).collect();

            // find the candidates and then check if actually intersecting
            for cand_index in indexes.clone() {
                criterion::black_box(geom.intersects(&all_polys[cand_index]));
            }
        });
    });
    c.bench_function("Naive", |bencher| {
        bencher.iter(|| {
            all_polys.iter().for_each(|poly| {
                criterion::black_box(geom.intersects(poly));
            });
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
