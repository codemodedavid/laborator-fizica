use std::io;

struct Data {
    temp: f64,
    res: f64,
    temp_k: f64,
    temp_1k: f64,
    ln_r: f64,
}
impl Data {
    fn new(temp: f64, res: f64) -> Self {
        let temp_k = temp + 273.15;
        let temp_1k = 1.0/temp_k;
        let ln_r = res.ln();
        Self {temp, res, temp_k, temp_1k, ln_r}
    }
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim()
        .parse() // parse the string into a number
        .expect("Please type a valid number")
}

fn main() {
    let mut data_vec: Vec<Data> = Vec::new();
    println!("ACEST PROGRAM CALCULEAZA DATELE NECESARE COMPLETARII TABELULUI LUCRARII TERMISTOR:\n");
    for i in 0..10 {
        let mut input = String::new();
        println!("temperatura {}:", i+1);

        let temp = read_number();
        ///////////////////////////////////
        println!("rezistenta {}:", i+1);

        let res = read_number();
        //////////////////////////////////
        
        let data = Data::new(temp, res);
        println!("\nkelvin: {:.3}", data.temp_k);
        println!("1/kelvin: {:.3}x10^-3", data.temp_1k * 1000.0);
        println!("ln(R): {:.3}", data.ln_r);
        
        data_vec.push(data);

        println!("\n//////////////////////////////\n");
    }
    let n = data_vec.len() as f64;

    let sum_x: f64 = data_vec.iter().map(|d| d.temp_1k).sum();
    let sum_y: f64 = data_vec.iter().map(|d| d.ln_r).sum();
    let sum_xy: f64 = data_vec.iter().map(|d| d.temp_1k * d.ln_r).sum();
    let sum_x2: f64 = data_vec.iter().map(|d| d.temp_1k.powi(2)).sum();

    let m = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
    let b = (sum_y - m * sum_x) / n;

    let k_b = 8.617e-5; // eV/K
    let delta_e = 2.0 * k_b * m;

    println!("Panta (m) = {:.5}", m * 0.001);
    println!("Intercept (b) = {:.5}", b);
    println!("Delta E (eV) = {:.5}", delta_e);
}