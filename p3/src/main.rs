use std::io;

// szyfr cezara uproszczony jak najbardziej jak szlo, bo rust things ... 
fn zadanie1(a: &str){
    let mut tab_char: Vec<char> = a.chars().collect();
    
    for _ in 1..11{
        for i in 0..10 {
            if tab_char[i] != ' '{
                let mut b: u32 = tab_char[i].into();
                b -= 1;
                tab_char[i] = b.try_into().unwrap();
            }
        }
        for i in 0..10 {
            print!("{}", tab_char[i]);
        }
        println!();
    }
}


//1,22,333,4444 etc
fn zadanie2(a: u16) -> u32{
    let mut sum: u32 = 0;
    let mut pot_ten: u32;
    let mut liczba: u32;

    for i in 1..a+1{
        let v = i as u32;
        liczba = v;
        pot_ten = 1;
        for _ii in 1..i{
            pot_ten *= 10;
            liczba += v*pot_ten;
        }
        println!("{}", liczba);
        sum += liczba;
    }
    return sum;
}


// zwraca najmniejsza liczbe podzielna przez 7 i reszta z dzielenia =1
fn zadanie4() -> u16 {
    let mut x: u16 = 7;
    while x < 2000 {
        if x % 2 == 1 && x % 3 == 1 && x % 4 == 1 && x % 5 == 1 && x % 6 == 1 { 
            return x;
        }
        x += 7;
    }
    return 0;
}

// zadanie 5, zliczanie wszystkich liczb powyzej
fn zadanie5() -> u16 {
    let mut x: u16 = 7;
    let mut count = 0;
    while x <= 2000 {
        if x % 2 == 1 && x % 3 == 1 && x % 4 == 1 && x % 5 == 1 && x % 6 == 1 { 
            count += 1;
        }
        x += 7;
    }
    return count;
}


// tabliczka mnozenia
fn zadanie8() {
    for i in 1..11{
        for j in 1..11 {
            print!("{}\t", i*j);
        }
        println!();
    }
}

fn main() {
    // zadanie 1
    println!("\nZadanie 1");
    zadanie1("Rmgi$ he}%");
    println!();

    // zadanie 2
    println!("\nZadanie 2\n");
    println!("{}", zadanie2(4));
    println!("{}", zadanie2(6));
    
    // zadanie3
    println!("\nZadanie 3");
    let n = 5;
    let mut iloczyn_1 = 1;
    let mut iloczyn_2 = 1;
    let mut iloczyn_3 = 1.;
    let mut iloczyn_4 = 1;
    let mut iloczyn_5 = 1.;

    for i in 1..n {
        iloczyn_1 *= i;
        iloczyn_2 *= 2*i-1;
        let ii = i as f32;
        iloczyn_3 *= 1./(ii*(ii+1.));
        iloczyn_4 *= i*i*i;
        iloczyn_5 *= 1./(ii * ii);
    }
    println!("Wynik pierwszego wzoru: {}\nDrugiego wzoru: {}\nTrzeci wzor: {:.2e}\nSzescian n liczb: {}\nOdwrotnosc kwadratow: {:.2e}", iloczyn_1, iloczyn_2, iloczyn_3, iloczyn_4, iloczyn_5);
    
    // zadanie 4
    println!("\nZadanie 4");
    println!("Najmniejsza liczba podzielna przez 7: {}", zadanie4());
    
    // zadanie 5
    println!("\nZadanie 5");
    println!("Ilosc liczb w przedziale [1,2000] podzielnych przez 7 + warunek wynosi: {}", zadanie5());

    
    //zadanie 6a
    println!("\nZadanie 6a");
    for i in 1..12 {
        let x = i;
        if 2*(2*x-12)-12 == 0 {
            println!("Na poczatku mial {} denarow", i);
            break;
        }
    }
    // zadanie 6b
    println!("\nZadanie 6b");
    let mut i = 1.;
    while i <= 12. {
        let mut x = i;
        for _j in 0..3 {
            x *= 2.;
            x -= 12.;
        }
        if x == 0. {
            println!("Na poczatku mial {} denarow", i);
        }
        i += 0.5;
    }


    // zadanie 7
    println!("\nZadanie 7");
    let mut a1 = 1;
    let mut a2 = 1;
    print!("{a1} ");
    while a2 <= 100 {
        print!("{a2} ");
        let temp = a2;
        a2 = a1+a2;
        a1 = temp;
    }
    println!();

    // zadanie 8
    println!("\nZadanie 8\n");
    zadanie8();


    // zadanie 9
    println!("\nZadanie 9");
    let mut do_dzielenia = 1.;
    let mut x = 1.;
    let mut i = 1.;
    
    while i < 15. {
        i += 1.;
        
        if x >= 1000. {
            do_dzielenia *= 10.;
            x = x*i/do_dzielenia;
        }
        else {
            x *= i;
        }
    }
    println!("Pierwsze 3 cyfry 15!: {}", x as i16);


    // zadanie 10
    println!("\nZadanie 10");
    let mut input = String::new();
    println!("Wpisz cos, program wymowi tylko cyfry:");
    //read user input into String
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!();
    for c in input.chars(){
        match c {
            '0' => {print!("zero ");}
            '1' => {print!("jeden ")}
            '2' => {print!("dwa ")}
            '3' => {print!("trzy ")}
            '4' => {print!("cztery ")}
            '5' => {print!("piec ")}
            '6' => {print!("szesc ")}
            '7' => {print!("siedem ")}
            '8' => {print!("osiem ")}
            '9' => {print!("dziewiec ")}
            '+' => {print!("plus ")}
            '-' => {print!("minus ")}
            _ => {} 
        }
    }
    println!();

    // zadanie 12
    println!("\nZadanie 12\n");
    for j in 1..6 {
        for _i in j..5 {
            print!(" ");
        }
        for _i in 1..2*j {
            print!("*");
        }
        println!();
    }
}
