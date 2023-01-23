#[macro_use]
extern crate criterion;
extern crate geo;
extern crate spatialindex;

use crate::geo::{BoundingRect, Intersects, Polygon};
use criterion::Criterion;
use rstar::{RTree, AABB};
use spatialindex::PolyWithIndex;
use std::io::BufRead;
use std::{fs::File, io::BufReader};
use wkt::TryFromWkt;

fn criterion_benchmark(c: &mut Criterion) {
    // read geometries from a text file
    let f = File::open("geoms.txt").expect("this shit to work");
    let f = BufReader::new(f);

    // create a vector of polygons
    let all_polys = f
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let line = line.expect("Unable to read line");
            let ply: Polygon<f64> = Polygon::try_from_wkt_str(line.as_str()).unwrap();
            PolyWithIndex::new(ply, idx)
        })
        .collect::<Vec<PolyWithIndex>>();

    let geom = all_polys[0].geom().clone();
    // create the tree
    let ap = all_polys.clone();
    let r_tree: RTree<_> = RTree::bulk_load(ap);

    c.bench_function("Tree-assisted", |bencher| {
        bencher.iter(|| {
            let rect = all_polys[0].clone().geom().bounding_rect().unwrap();
            let bbox = [[rect.min().x, rect.min().y], [rect.max().x, rect.max().y]];
            // calculate candidates
            let intersect_candidates = r_tree.locate_in_envelope_intersecting(&AABB::from_corners(
                bbox[0].into(),
                bbox[1].into(),
            ));
            intersect_candidates.for_each(|poly| {
                // check for intersection
                criterion::black_box(geom.intersects(poly.geom()));
            });
        });
    });
    c.bench_function("Naive", |bencher| {
        bencher.iter(|| {
            all_polys.iter().for_each(|poly| {
                // check for intersection
                criterion::black_box(geom.intersects(poly.geom()));
            });
        });
    });
    c.bench_function("Known to intersect", |bencher| {
        bencher.iter(|| {
            [0usize, 47, 49, 91, 93].iter().for_each(|idx| {
                criterion::black_box(geom.intersects(all_polys[*idx].geom()));
            });
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
