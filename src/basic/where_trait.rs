trait Calculator {
    fn calculate(&self) -> f64;
}

impl<T> Calculator for T
where
    // add Copy trait to allow for coping of the value
    T: Into<f64> + Copy,
{
    fn calculate(&self) -> f64 {
        // unwrap the value and convert it to f64
        let num: f64 = (*self).into();
        num * num + 2.0 * num + 1.0
    }
}

pub fn test() {
    let num = 5;
    println!("calculate({}) result: {}", num, num.calculate());
}
