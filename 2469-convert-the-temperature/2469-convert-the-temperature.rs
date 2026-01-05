impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let kelvin: f64 = celsius + 273.15;
        let fahrenheit: f64 = celsius * 1.80 + 32.00;
        [kelvin, fahrenheit].to_vec()
    }
}