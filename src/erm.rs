use geo_types::{Polygon, MultiPolygon};
use rstar::primitives::GeomWithData;
use rstar::{RTreeNode, ParentNode};
use geo::BooleanOps;

pub fn inner(papa: &ParentNode<GeomWithData<Polygon, usize>>) -> MultiPolygon {
    papa
        .children()
        .iter()
        .fold(MultiPolygon::new(vec![]),  |accum, child| 
            match child {
                RTreeNode::Leaf(value) => {
                    let v = MultiPolygon::try_from(value.geom().to_owned()).unwrap();
                    accum.union(&v)
            },
            RTreeNode::Parent(parent) => {
                let value = inner(parent);
                value
            }
        })
}
