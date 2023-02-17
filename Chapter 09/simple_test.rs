#[test]
fn is_it_two() {
    assert_eq!(1+1, 2)
}
#[test]
#[ignore = "this test isn't implemented yet"]
fn not_ready(){
    // ...
}

#[test]
#[should_panic = "19 and 21 are not the same"]
fn they_arent_equal(){
    assert_eq!(19, 21, "19 and 21 are not the same")
}