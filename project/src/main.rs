//! Main module for the ABDS210 Final Project.
//! This module processes museum data to predict revenue ranges using a supervised machine learning model.


use ndarray::{Array2, Array, ArrayView1, array};
use linfa_trees::DecisionTree;
use linfa::prelude::*;

use libs::*;

///data_to_predict() puts data in form (array) to predict revenue using decision tree model
///input: museum type, locale, state, region codes as they would appear in the dataset. 
///The location demographics are already in numbered categories 
///(and we can find the keys online as the dataset states the numbering systems it uses)
///The museum type must be converted to numbered categories
///output: array that can be used by the decision tree to predict revenue
fn data_to_predict(mtype: String, m_locale: i8, m_state: i8, m_region: i8) -> Array2<f32> {
  let type_code = match mtype.as_str() {
    "HISTORIC PRESERVATION" => 1,
    "GENERAL MUSEUM" => 2,
    "ART MUSEUM" => 3,
    "HISTORY MUSEUM" => 4,
    "ARBORETUM, BOTANICAL GARDEN, OR NATURE CENTER" => 5,
    _ => 0,
  }; let new: Array2<f32> = array![[type_code as f32, m_locale as f32, m_state as f32, m_region as f32]];
  return new
}

///formats the prediction from the model to an understandable sentence with the predicted revenue range
///inputs: fences vector (the determined discrete revenue categories), predicted_class: outcome of decision tree prediction
///output: string to be read in the terminal
fn format_categories(fences: &Vec<f64>, predicted_class: ArrayView1<usize>) -> String {
  let cat_index = predicted_class[0];
  if cat_index == 0 {
    format!("Predicted revenue range: < ${}", fences[cat_index])
  } else if cat_index < fences.len() {
    format!("Predicted revenue range: ${} - ${}", fences[cat_index-1], fences[cat_index])
  } else {
    format!("Predicted revenue range: > ${}", fences[cat_index - 1])
  }
}

fn main() {
  //read the csv, get the data and make it usable, create a decision tree
  //the loading_data module is utilized here
  let mut filtered: Vec<libs::loading_data::Row> = Vec::new();
  if let Ok(filtered_rows) = loading_data::load("museums.csv") {
      filtered = filtered_rows;
  };
  let mut nums: Vec<f32> = Vec::new();
  let mut rev: Vec<f64>= Vec::new();
  //getting data ready to be put into arrays (from vectors) which will be made into a Dataset (from linfa crate)
  for row in &filtered {
    nums.push(row.museum_type as f32);
    nums.push(row.locale as f32);
    nums.push(row.state_code as f32);
    nums.push(row.region_code as f32);
    rev.push(row.revenue);
  };

  // putting the revenue into discrete categories, I experimented with different fences
  // the categorize_rev module is utilized here 
  let fences = vec![0.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1000000000.0];
  let mut revenue_cats: Vec<usize> = Vec::new();
  for elem in rev {
    revenue_cats.push(categorize_rev::categorize(elem, &fences));
  }

  let array1 = Array::from_vec(revenue_cats);
  let array2 = Array2::from_shape_vec((filtered.len(), 4), nums).expect("Error creating ndarray");
  //the revenue array is the targets
  let dataset = Dataset::new(array2, array1).with_feature_names(vec!["Museum Type", "Locale", "State", "Region"]);
  //decision tree from linfa crate - supervised mla
  let decision_tree = DecisionTree::params().max_depth(Some(4)).fit(&dataset).unwrap();
  //calculate accuracy with confusion matrix
  let accuracy = decision_tree.predict(&dataset).confusion_matrix(&dataset).unwrap().accuracy();
  println!("The accuracy is: {:?}", accuracy);


  //taking data from the kaggel file that are missing revenue and using the model to try to predict the revenue:
  //predict revenue for ALASKA HISTORICAL MUSEUM
  let ahm = data_to_predict("HISTORIC PRESERVATION".to_string(), 1, 2, 6); 
  let prediction1 = decision_tree.predict(&ahm);
  let predicted_class1 = prediction1.as_targets();
  println!("{}", format_categories(&fences, predicted_class1.view()));

  //predict revenue for BARBER VINTAGE MOTORSPORT MUSEUM
  let bvmm = data_to_predict("GENERAL MUSEUM".to_string(), 4,1,3);
  let prediction2 = decision_tree.predict(&bvmm);
  let predicted_class2 = prediction2.as_targets();
  println!("{}", format_categories(&fences, predicted_class2.view()));

  //predict revenue for COOKS NATURAL SCIENCE MUSEUM
  let cnsm = data_to_predict("SCIENCE & TECHNOLOGY MUSEUM OR PLANETARIUM".to_string(), 1,1,3);
  let prediction3 = decision_tree.predict(&cnsm);
  let predicted_class3 = prediction3.as_targets();
  println!("{}", format_categories(&fences, predicted_class3.view()));
}

#[test]
fn test_index_rev() {
  let test_fences = vec![0.0,10.0,20.0,30.0];
  let arraya = Array::from(vec![0]);
  let a: ArrayView1<usize> = arraya.view();
  let arrayb = Array::from(vec![1]);
  let b: ArrayView1<usize> = arrayb.view();
  let arrayc = Array::from(vec![2]);
  let c: ArrayView1<usize> = arrayc.view();
  let arrayd = Array::from(vec![3]);
  let d: ArrayView1<usize> = arrayd.view();
  let arraye = Array::from(vec![4]);
  let e: ArrayView1<usize> = arraye.view();
  assert_eq!(format_categories(&test_fences, a), String::from("Predicted revenue range: < $0"));
  assert_eq!(format_categories(&test_fences, b), String::from("Predicted revenue range: $0 - $10"));
  assert_eq!(format_categories(&test_fences, c), String::from("Predicted revenue range: $10 - $20"));
  assert_eq!(format_categories(&test_fences, d), String::from("Predicted revenue range: $20 - $30"));
  assert_eq!(format_categories(&test_fences, e), String::from("Predicted revenue range: > $30"));
}
