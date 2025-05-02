use ndarray::Array2;
use ndarray::Array;
use linfa_trees::DecisionTree;
use linfa::prelude::*;

use libs::loading_data;



/*let cat = match rev {
  >.5bil
  .5bil to 10mil
  10 mil to 1 mil
  1 mil to 100 thou
  90 thou
  80thou
  
}*/
fn main() {
  let mut filtered: Vec<libs::loading_data::Row> = Vec::new();
  if let Ok(filtered_rows) = loading_data::load("museums.csv") {
      filtered = filtered_rows;
      //println!("{:?}",&filtered.len())
  };
  //let mut numbs: Vec<i64> = Vec::new();
  let mut nums: Vec<f32> = Vec::new();
  //let mut strs: Vec<&str> = Vec::new();
  let mut rev: Vec<usize>= Vec::new();
  //println!("{:?}",filtered.len());
  for row in &filtered {
    //nums.push(row.museum_id);
    //strs.push(&row.museum_name);
    nums.push(row.museum_type as f32);
    nums.push(row.locale as f32);
    nums.push(row.state_code as f32);
    nums.push(row.region_code as f32);
    rev.push(row.revenue);
  };
  //println!("{:?}", rev);

  let array1 = Array::from_vec(rev);
  let array3 = Array2::from_shape_vec((filtered.len(), 4), nums).expect("Error creating ndarray");
  let dataset = Dataset::new(array3, array1).with_feature_names(vec!["Museum Type", "Locale", "State", "Region"]);
  
  let decision_tree = DecisionTree::params().max_depth(Some(4)).fit(&dataset).unwrap();

  let accuracy = decision_tree.predict(&dataset).confusion_matrix(&dataset).unwrap().accuracy();
    
  println!("The accuracy is: {:?}", accuracy);
}

