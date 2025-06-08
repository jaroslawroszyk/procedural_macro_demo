use procedural_macro::RetryCalculation;

trait Calc {
    fn calc_count(&self) -> usize;
}

#[derive(RetryCalculation)]
#[calculation = "not-a-number"]
struct MyCalc; 