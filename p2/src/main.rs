fn max(a: f64, b: f64) -> f64 {
    if a > b {return a;}
    else {return b;}
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {return a;}
    else {return b;}
}

fn main() {
    // Zadanie 1
    println!("\nZadanie 1");
    let mut x = 10.;
    // funkcja heaviside'a
    println!("Funkcja Heaviside'a");
    if x >= 0. {
        println!("1");
    }
    else {
        println!("0");
    }
    // abs
    println!("Wartosc bezwzgledna");
    if x >= 0. {
        println!("{x}");
    }
    else {
        println!("{}", -1. * x);
    }
    //sgn
    println!("sgn(x)");
    if x >= 0. {
        println!("1");
    }
    else {
        println!("-1");
    }

    // Zadanie 2
    println!("\nZadanie 2");
    let tab = [1.,3.,3.];
    let mut sum = 0.;
    for x in 0..3 {
        sum += tab[x];
    }
    let biggest = max(tab[0], max(tab[1], tab[2]));
    if sum - biggest > biggest {
        println!("Mozna stworzyc trojkat");
    }
    else {
        println!("Nie mozna stworzyc trojkata");
    }

    // Zadanie 3
    // za duzo maty, nie chce mi sie

    // Zadanie 4
    println!("\nZadanie 4");
    let mut y = 2.;
    x = 3.;
    if x > 0. && y > 0. {
        println!("Punkt nalezy do pierwszej cwiartki");
    }
    else if x < 0. && y > 0. {
        println!("Punkt nalezy do drugiej cwiartki");
    }
    else if x < 0. && y < 0. {
        println!("Punkt nalezy do trzeciej cwiartki");
    }
    else if x > 0. && y < 0. {
        println!("Punkt nalezy do czwartej cwiartki");
    }
    else if x == 0. || y == 0. {
        println!("Punkt znajduje sie na granicy cwiartek");
    }

    // Zadanie 5
    println!("\nZadanie 5");
    println!("Przesuniecie bitowe o 3 bity w prawo liczby: {} wynosi: {}", 127, 127>>3);
    println!("Przesuniecie bitowe o 3 bity w lewo liczby: {} wynosi: {}", 127, 127<<3);

    // Zadanie 6
    println!("\nZadanie 6");
    let s = "Ala ma kota";
    for wyraz in s.split(" ") {
        println!("{}", wyraz);
    }

    // Zadanie 7
    println!("\nZadanie 7");
    let pesel = "05320815134";
    let nr_plec = pesel.chars().nth(9).unwrap();
    let int_nr_plec = nr_plec.to_digit(10).unwrap();
    if int_nr_plec % 2 == 1 {
        println!("Osoba jest mezczyzna");
    }
    else {
        println!("Osoba jest kobieta");
    }
    let pierwsza_cyfra = pesel.chars().nth(0).unwrap().to_digit(10).unwrap();
    let druga_cyfra = pesel.chars().nth(1).unwrap().to_digit(10).unwrap();
    let trzecia_cyfra = pesel.chars().nth(2).unwrap().to_digit(10).unwrap();
    let czwarta_cyfra = pesel.chars().nth(3).unwrap().to_digit(10).unwrap();
    
    let tab_dzien = [pesel.chars().nth(4).unwrap(), pesel.chars().nth(5).unwrap()];
    let dzien: String = tab_dzien.iter().collect();
    // glupie sa te zadania

    // Zadanie 8
    println!("\nZadanie 8");
    let liczba = 8;
    println!("Testowana liczba decymalnie: {}\nBin: {:b}\nHex: {:x}\nOct: {:o}", liczba, liczba, liczba, liczba);
}
