// Sample could be an aggregate of multiple samples, not neccessarily just one sample
#[derive(Clone, Debug)]
pub struct Sample {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub count: usize,
}
