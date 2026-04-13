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
    let m = 9.109e-31;
    let h = 6.625e-34;

    let d2 = 1.23e-10;
    let d1 = 2.13e-10;

    let l = 13.5;

    let mut diametru1;
    let mut diametru2;

    let mut lambda1;
    let mut lambda2;
    let mut lambdat;
    
    let mut tensiune;
    let mut supratensiune;
    

    println!("ACEST PROGRAM CALCULEAZA DATELE NECESARE COMPLETARII TABELULUI LUCRARII DEBYE:\n");

    loop {
        println!("Introduce valoarea tensiunii (kV)");
        tensiune = read_number_f();
        tensiune *= 1000.0;
        println!("Introduce valoarea Diametrului 1 (cm)");
        diametru1 = read_number_f();
        //diametru1 /= 100.0;
        println!("Introduce valoarea Diametrului 2 (cm)");
        diametru2 = read_number_f();
        //diametru2 /= 100.0;

        lambda1 = d1 * diametru1 / (2.0 * l);
        lambda2 = d2 * diametru2 / (2.0 * l);

        let numar = 2.0 * m * e * tensiune;
        lambdat = h / numar.sqrt();
        
        supratensiune = 1.0 / tensiune.sqrt();

        println!("\nREZULTATE FINALE:");
        println!("1/U: {:.4}", supratensiune);
        println!("lambda 1 {:.4}e-12 (pm)", lambda1 * 1e12);
        println!("lambda 2: {:.4}e-12 (pm)", lambda2 * 1e12);
        println!("lambda teoretic: {:.4}e-12 (pm)", lambdat * 1e12);
        println!("\n");
    }
}

