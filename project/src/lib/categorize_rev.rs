//! makes revenue a discrete variable from a continuous variable

//the categorize function turns revenue, a continuous variable, to a discrete variable by specifying bins (like a histogram)
//for each revenue in the data, putting that number through the categorize function will assign it to a bin
//inputs: revenue (just one datapoint), fences vector which defines the bins
//output: revenue category (the number it outputs is the upper bound of the bin)
pub fn categorize(money: f64, fences: &Vec<f64>) -> usize {
    //sort the fences vector using partial_cmp as the fences are f64 values, this accounts for situations where the fences are not in order
    let mut sorted_fences = fences.clone();
    sorted_fences.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // loop through the fences to find where the input lies, should return one more than the last index if the value is greater than the largest fence/bound
    for i in 0..fences.len() {
        if money < sorted_fences[i] {
            return i;
        }
    } if let Some(last) = sorted_fences.last() {
        if money >= *last {
            return fences.len();
        }
    } return usize::MAX //if something goes wrong
}


#[test]
fn test_unordered_fences() {
    let fences = vec![100.0, 200.0, 150.0];
    let money = 120.0;
   
    let a = categorize(money, &fences);
    let mut sorted = fences.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let b = categorize(money, &sorted);
    
    assert_eq!(a,b,"fences should be sorted");
}

#[test]
fn test_categorize_revenue() {
    let fences = vec![0.0, 1000.0, 10000.0, 100000.0, 1000000.0, 1000000000.0];

    assert_eq!(categorize(-10.0, &fences), 0);
    assert_eq!(categorize(0.0, &fences), 1);
    assert_eq!(categorize(999.99, &fences), 1);
    assert_eq!(categorize(1000.0, &fences), 2);
    assert_eq!(categorize(9999.99, &fences), 2);
    assert_eq!(categorize(10000.0, &fences), 3);
    assert_eq!(categorize(99999.99, &fences), 3);
    assert_eq!(categorize(100000.0, &fences), 4);
    assert_eq!(categorize(999999.99, &fences), 4);
    assert_eq!(categorize(1000000.0, &fences), 5);
    assert_eq!(categorize(999999999.99, &fences), 5);
    assert_eq!(categorize(1000000000.0, &fences), 6);
}
