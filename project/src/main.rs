use ndarray::Array2;
use ndarray::Array;
use linfa_trees::DecisionTree;
use linfa::prelude::*;

use libs::loading_data;
use libs::categorize_rev;


fn main() {
  let mut filtered: Vec<libs::loading_data::Row> = Vec::new();
  if let Ok(filtered_rows) = loading_data::load("museums.csv") {
      filtered = filtered_rows;
      //println!("{:?}",&filtered.len())
  };
  //let mut numbs: Vec<i64> = Vec::new();
  let mut nums: Vec<f32> = Vec::new();
  //let mut strs: Vec<&str> = Vec::new();
  let mut rev: Vec<f64>= Vec::new();
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

  //categorizing
  let fences = vec![0.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1000000000.0];
  let mut revenue_cats: Vec<usize> = Vec::new();
  for elem in rev {
    revenue_cats.push(categorize_rev::categorize(elem, &fences));
  }


  let array1 = Array::from_vec(revenue_cats);
  let array3 = Array2::from_shape_vec((filtered.len(), 4), nums).expect("Error creating ndarray");
  let dataset = Dataset::new(array3, array1).with_feature_names(vec!["Museum Type", "Locale", "State", "Region"]);
  
  let decision_tree = DecisionTree::params().max_depth(Some(3)).fit(&dataset).unwrap();

  let accuracy = decision_tree.predict(&dataset).confusion_matrix(&dataset).unwrap().accuracy();
    
  println!("The accuracy is: {:?}", accuracy);
}

