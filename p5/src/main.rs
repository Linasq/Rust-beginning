use std::{collections::HashMap, io};

fn main() {
    // Zadanie 1
    // print first letter of name, surname and persons phone number
    println!("\nZadanie 1");
    let mut tel = HashMap::new();
    tel.insert(("Jan", "Kowalski"), "+48 123123123");
    tel.insert(("Anna", "Nowak"), "+48 890890890");
    tel.insert(("Mateusz", "Botak"), "+48 789243678");
    
    let mut imie;
    let mut nazwisko;
    for (key, value) in tel.into_iter() {
        imie = key.0;
        nazwisko = key.1;
        println!("{}{} nr {}", imie.chars().nth(0).unwrap(), nazwisko.chars().nth(0).unwrap(), value);
    }
    
    // Zadanie 2
    // input year, find out who was born in that year
    println!("\nZadanie 2");
    let mut urodziny = HashMap::new();
    urodziny.insert(("Jan", "Kowalski"), "12.12.2012");
    urodziny.insert(("Anna", "Nowak"), "04.03.2002");
    urodziny.insert(("Mateusz", "Botak"), "01.01.2002");

    println!("Podaj rok do wypisania: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error while reading line");
    let input_trimmed = input.trim();

    for (key, value) in urodziny.into_iter() {
        let data = value.split(".");
        let collection: Vec<&str> = data.collect();
        let rok = collection[2];

        if rok == input_trimmed {println!("Ta osoba sie urodzila w {} roku: {} {}", input,key.0, key.1);}
    }

    // Zadanie 3
    // input name of organisation, find out its alarm number
    println!("\nZadanie 3");
    let mut nr_alarmowe = HashMap::new();
    nr_alarmowe.insert("wszystko", 112);
    nr_alarmowe.insert("straz", 998);
    nr_alarmowe.insert("policja", 997);
    nr_alarmowe.insert("pogotowie", 999);
    
    // stworzenie tablicy dynamicznej, ktora ma tuple powstale z slownika
    let mut posortowane: Vec<_> = nr_alarmowe.clone().into_iter().collect();

    // uzycie domkniecia (lambdy) by posortowac wg wartosci
    posortowane.sort_by(|x,y| x.1.cmp(&y.1));
    for value in posortowane {
        println!("{} ma nr {}", value.0, value.1);
    }
    let nr_alarmowe_v2 = nr_alarmowe.clone();
    println!("Kopia nr alarmowych: {:?}", nr_alarmowe_v2);
    println!("\nTeraz podaj nazwe jednostki (lub wszystko), a my wypiszemy jej nr alarmowy");
    let mut input_z3 = String::new();
    io::stdin().read_line(&mut input_z3).expect("Error while reading line");
    let input_z3_trimmed = input_z3.trim();

    match input_z3_trimmed {
        "wszystko"  => {println!("Twoj nr alarmowy to: {}", nr_alarmowe.get(&"wszystko").unwrap());}
        "straz"     => {println!("Twoj nr alarmowy to: {}", nr_alarmowe.get(&"straz").unwrap());}
        "pogotowie" => {println!("Twoj nr alarmowy to: {}", nr_alarmowe.get(&"pogotowie").unwrap());}
        "policja"   => {println!("Twoj nr alarmowy to: {}", nr_alarmowe.get(&"policja").unwrap());}
        _           => {println!("Nie znaleziono takiej jednostki");}
    }
}
