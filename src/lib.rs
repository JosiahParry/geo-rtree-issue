use geo::geometry::Polygon;
use rstar::primitives::GeomWithData;

/// A [Polygon] and its original index
pub type PolyWithIndex = GeomWithData<Polygon, usize>;

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{BoundingRect, Intersects, Polygon};
    use rstar::{RTree, AABB};
    use std::io::BufRead;
    use std::{fs::File, io::BufReader};
    use wkt::TryFromWkt;

    #[test]
    fn tree_vs_naive() {
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
        let rect = all_polys[0].clone().geom().bounding_rect().unwrap();
        let bbox = [[rect.min().x, rect.min().y], [rect.max().x, rect.max().y]];
        let mut tree_intersection_indices = Vec::new();
        let mut naive_intersection_indices = Vec::new();

        // calculate tree candidates
        let intersect_candidates = r_tree
            .locate_in_envelope_intersecting(&AABB::from_corners(bbox[0].into(), bbox[1].into()));
        intersect_candidates.for_each(|poly| {
            // check for intersection
            if geom.intersects(poly.geom()) {
                tree_intersection_indices.push(poly.data)
            }
        });
        // calculate naive intersection indices
        all_polys.iter().for_each(|poly| {
            // check for intersection
            if geom.intersects(poly.geom()) {
                naive_intersection_indices.push(poly.data)
            }
        });
        // sort indices
        tree_intersection_indices.sort_unstable();
        naive_intersection_indices.sort_unstable();
        assert_eq!(&tree_intersection_indices, &naive_intersection_indices)
    }
}
