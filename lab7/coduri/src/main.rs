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
    let e = 1.602e-19;
    let mut n;
    let mut data_t: Vec<f64> = Vec::new();
    let mut data_t_avg: Vec<f64> = vec![0.0; 4];
    let mut data_freq: Vec<f64> = vec![0.0; 4];
    println!("ACEST PROGRAM CALCULEAZA DATELE NECESARE COMPLETARII TABELULUI LUCRARII PLANCK:\n");

    'filter_loop: loop {
        println!("Alege filtrul de culoare (1 / 2 / 3 / 4)");
        let filtru = read_number();
        let (lambda, freq) = match filtru {
            1 => (578, 5190e11),
            2 => (546, 5494e11),
            3 => (436, 6883e11),
            4 => (405, 7412e11),
            _ => {
            println!("filtru invalid. alege 1 (galben), 2 (verde), 3 (albastru), 4 (violet).");
            continue;
            }
        }; 

        println!("Introduce numarul de masuratori:");
        n = read_number();
        
        for i in 0..n {
            println!("tensiunea {}:", i+1);
                
            let tensiune = read_number_f();
            data_t.push(tensiune);

            
        }
        let mean: f64 = data_t.iter().sum::<f64>() / n as f64;
        data_t_avg[filtru as usize -1] = mean;
        data_freq[filtru as usize -1] = freq;

        let sum_sq_diff: f64 = data_t.iter().map(|&x| (x - mean).powi(2)).sum();
        let sigma = (sum_sq_diff / (n * (n - 1)) as f64).sqrt();
        println!("//////////////////");
        println!("U{} = {:.3} ± {:.7}", filtru, mean, sigma);
        println!("//////////////////");
        data_t.clear();

        loop {
            println!("GATA PENTRU CALCUL FINAL? (0 / 1)");
            let input = read_number();

            match input {
                0 => continue 'filter_loop,
                1 => break 'filter_loop,
                _ => {
                    println!("input invalid. alege 0 sau 1.");
                    continue;
                }
            }
        }
    }
    let mut h_sum = 0.0;
    for i in 0..4 {
        for j in (i + 1)..4 {
            let h = e * ((data_t_avg[j] - data_t_avg[i]) / (data_freq[j] - data_freq[i]));
            println!("h{}{}: {:.4}e-34", i+1, j+1, h * 1e34);
            h_sum += h;
        }
    }
    println!("\nREZULTATE FINALE:");
    println!("h: {:.4}e-34", h_sum / 6.0 * 1e34);
    println!("m: {:.4}e-15", h_sum / 6.0 * 1e34 / e * 1e-19);
}
