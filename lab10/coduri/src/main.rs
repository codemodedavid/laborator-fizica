use std::io;

fn read_number_f() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim()
        .parse() // parse the string into a number
        .expect("Please type a valid number")
}
fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim()
        .parse() // parse the string into a number
        .expect("Please type a valid number")
}

fn main() {
    let mut data_r: Vec<f64> = Vec::new();

    println!("ACEST PROGRAM CALCULEAZA DATELE NECESARE COMPLETARII TABELULUI LUCRARII RYDBERG:\n");

    loop {
        let labels = ["alpha", "beta", "gamma", "delta", "epsilon", "zeta"];

        println!("Introduce valorile pentru seria Balmer:\n");

        for i in 0..6 {
            let n = i as f64 + 3.0;

            println!("Introduce X pentru H_{}:", labels[i]);

            let mut x = read_number_f();

            // calibrare experimentala
            x = x * -0.1416 + 8.5166;

            println!("1/lambda^2 = {:.4} um^-2", x);

            // λ în micrometri (μm)
            let lambda_um = 1.0 / x.sqrt();

            // conversii corecte
            let lambda_nm = lambda_um * 1000.0;
            let lambda_m = lambda_um * 1e-6;

            println!("lambda = {:.4} nm", lambda_nm);

            // formula corecta pentru constanta Rydberg
            let rydberg = 1.0 / (lambda_m * (1.0 / 4.0 - 1.0 / n.powi(2)));

            println!("constanta rydberg = {:.4e} m^-1\n", rydberg);

            data_r.push(rydberg);
        }

        // media
        let n_samples = data_r.len() as f64;
        let mean: f64 = data_r.iter().sum::<f64>() / n_samples;

        // abatere standard (eroare statistica)
        let sum_sq_diff: f64 = data_r.iter().map(|&v| (v - mean).powi(2)).sum();
        let sigma = (sum_sq_diff / (n_samples * (n_samples - 1.0))).sqrt();

        println!("/////////////////////////////");
        println!("Rydberg = {:.4e} ± {:.4e} m^-1", mean, sigma);
        println!("/////////////////////////////\n");

        data_r.clear();
    }
}