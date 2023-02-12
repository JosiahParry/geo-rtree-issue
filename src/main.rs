use geo::Geometry;
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::time::{Instant};

fn main() {

  // downloaded from https://azuremapscodesamples.azurewebsites.us/Common/data/geojson/parcels.json
  
    let start = Instant::now();
    let fp =  "/Users/josiahparry/Downloads/parcels.geojson";

    let file = std::fs::File::open(fp)
        .expect("this to work");
  
    let data = geojson::FeatureReader::from_reader(file);

    let res = data
        .features()
        .par_bridge()
        .into_par_iter()
        .map(|feat| feat.unwrap().properties.unwrap())
        .collect::<Vec<Map<String, Value>>>();

        let duration = start.elapsed();

        println!("Time elapsed is: {:?}", duration);

    // time reading geometries 
    let start = Instant::now();
    let fp =  "/Users/josiahparry/Downloads/parcels.geojson";

    let file = std::fs::File::open(fp)
        .expect("this to work");
  
    let data = geojson::FeatureReader::from_reader(file);

    let res = data
        .features()
        .par_bridge()
        .into_par_iter()
        .map(|feat| geo::Geometry::try_from(feat.unwrap().geometry.unwrap()).unwrap())
        .collect::<Vec<Geometry>>();

        let duration = start.elapsed();

        println!("Time elapsed is: {:?}", duration);
}



