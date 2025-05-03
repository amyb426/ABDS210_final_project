use ndarray::{Array2, Array, ArrayView1, array};
use linfa_trees::DecisionTree;
use linfa::prelude::*;

use libs::*;

//making a fn to put data in form (array) to predict revenue
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

fn format_categories(fences: &Vec<f64>, predicted_class: ArrayView1<usize>) {
  let cat_index = predicted_class[0];
  if cat_index + 1 < fences.len() {
    println!(
        "Predicted revenue range: ${} - ${}",
        fences[cat_index],
        fences[cat_index + 1]
    );
  } else {
    println!("Predicted revenue range is out of bounds!");
  }
}

fn main() {
  //read the csv, get the data and make it usable, create a decision tree
  let mut filtered: Vec<libs::loading_data::Row> = Vec::new();
  if let Ok(filtered_rows) = loading_data::load("museums.csv") {
      filtered = filtered_rows;
  };
  let mut nums: Vec<f32> = Vec::new();
  let mut rev: Vec<f64>= Vec::new();
  
  for row in &filtered {
    nums.push(row.museum_type as f32);
    nums.push(row.locale as f32);
    nums.push(row.state_code as f32);
    nums.push(row.region_code as f32);
    rev.push(row.revenue);
  };

  //putting the revenue into discrete categories, I experimented with different fences
  let fences = vec![0.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1000000000.0];
  let mut revenue_cats: Vec<usize> = Vec::new();
  for elem in rev {
    revenue_cats.push(categorize_rev::categorize(elem, &fences));
  }

  let array1 = Array::from_vec(revenue_cats);
  let array3 = Array2::from_shape_vec((filtered.len(), 4), nums).expect("Error creating ndarray");
  let dataset = Dataset::new(array3, array1).with_feature_names(vec!["Museum Type", "Locale", "State", "Region"]);
  
  let decision_tree = DecisionTree::params().max_depth(Some(4)).fit(&dataset).unwrap();

  let accuracy = decision_tree.predict(&dataset).confusion_matrix(&dataset).unwrap().accuracy();
    
  println!("The accuracy is: {:?}", accuracy);

  let ahm = data_to_predict("HISTORIC PRESERVATION".to_string(), 1, 2, 6); //predicting revenue for ALASKA HISTORICAL MUSEUM
  let prediction1 = decision_tree.predict(&ahm);
  let predicted_class1 = prediction1.as_targets();
  format_categories(&fences, predicted_class1.view());

  //predict revenue for BARBER VINTAGE MOTORSPORT MUSEUM
  let bvmm = data_to_predict("GENERAL MUSEUM".to_string(), 4,1,3);
  let prediction2 = decision_tree.predict(&bvmm);
  let predicted_class2 = prediction2.as_targets();
  format_categories(&fences, predicted_class2.view());

  //predict revenue for COOKS NATURAL SCIENCE MUSEUM
  let cnsm = data_to_predict("SCIENCE & TECHNOLOGY MUSEUM OR PLANETARIUM".to_string(), 1,1,3);
  let prediction3 = decision_tree.predict(&cnsm);
  let predicted_class3 = prediction3.as_targets();
  format_categories(&fences, predicted_class3.view());
}

