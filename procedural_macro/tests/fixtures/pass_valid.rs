use procedural_macro::RetryCalculation;

trait Calc {
    fn calc_count(&self) -> usize;
}

#[derive(RetryCalculation)]
#[calculation = 5]
struct MyCalc;

fn main() {
    let s = MyCalc;
    assert_eq!(s.calc_count(), 5);
}
