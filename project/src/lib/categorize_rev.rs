
pub fn categorize(money: f64, fences: &Vec<f64>) -> usize {
    let mut sorted_fences = fences.clone();
    sorted_fences.sort_by(|a, b| a.partial_cmp(b).unwrap());
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
    // Since the function assumes sorted fences, we expect the result to be misleading
    // This test checks that, or you can sort the fences before calling categorize
    let a = categorize(money, &fences);
    let mut sorted = fences.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let b = categorize(money, &sorted);
    assert_eq!(a,b,"fences should be sorted");
}
