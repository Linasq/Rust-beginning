use rand::Rng;

// it returns tab filled with random numbers
fn return_rng() -> Vec<u64>{
    let mut rng = rand::thread_rng();
    let mut tab: Vec<u64> = Vec::new();
    for _ in 0..10 {
        let num:u64 =  rng.gen_range(0..100);
        tab.push(num);
    }
    return tab;
}

// zadanie 7a, sum of numbers in tab
fn zadanie_7a(tab: Vec<u64>) -> u64{
    let mut sum: u64 = 0;
    for i in tab {
        sum += i;
    }
    return sum;
}

// zadanie 7b, product of numbers in tab
fn zadanie_7b(tab: Vec<u64>) -> u64{
    let mut product: u64 = 1;
    for i in tab {
        product *= i;
    }
    return product;
}


// zadanie 7c, mean
fn zadanie_7c(tab: Vec<u64>) -> f32 {
    let sum = zadanie_7a(tab.clone());
    let len = tab.len() as f32;
    let another_sum = sum as f32;
    let mean = another_sum / len;
    return mean;
}


// zadanie 7e
fn zadanie_7e(tab: Vec<u64>) -> u64 {
    let len = tab.len() as u64;
    let mut sum: u64 = 0;
    for _ in 0..len {
        let mut num: u64 = 1;
        for j in 1..len{
            let ind = j as usize;
            num *= tab[ind];
        }
        sum += num;
    }
    return sum;
}


// zadanie 8 binary search
fn zadanie_8(tab: Vec<u64>, szukana_liczba: u64) -> String {
    let n = tab.len() as usize;
    let mut a: usize = 0;
    let mut b: usize = n-1;
    let mut idx = (a+b)/2;

    while a != b {
        if szukana_liczba > tab[idx] {
            a = idx;
            idx = (a+b)/2;
        }
        else if szukana_liczba < tab[idx] {
            b = idx;
            idx = (a+b)/2;
        }

        if szukana_liczba == tab[idx] {
            return format!("Szukana liczba {szukana_liczba} znajduje sie na indeksie = {idx}");
        }
        if b-a == 1 && tab[a] != szukana_liczba {idx = (a+b+1)/2;}
        else {
            return format!("Nie znaleziono szukanej liczby: {}", szukana_liczba);
        }
    }
    return format!("Nie znaleziono szukanej liczby: {}", szukana_liczba);
}


// zadanie 9, find if there is a,b that a+b = x O(n)
fn zadanie_9(A: Vec<u64>, B: Vec<u64>, x: i32) -> String {
    let mut ida: usize = 0;
    let mut idb: usize = B.len();
    idb -= 1; // starting idx
    let mut sum: i32;
    let mut aa: i32;
    let mut bb: i32;

    while ida != 9 && idb != 0 {
        aa = A[ida] as i32;
        bb = B[idb] as i32;
        sum = x - aa - bb;
        if sum == 0 {return format!("\nUdalo sie znalezc takie liczby\nMusimy dodac {} z tablicy A oraz {} z tablicy B", A[ida], B[idb]);}
        else if  sum > 0 {ida += 1;}
        else {idb -= 1;}
    }
    return format!("Nie udalo sie znalezc liczb, ktorych suma wynioslaby: {x}"); 
}

fn main() {
    // zadanie 1
    println!("\nZadanie 1");
    let tab_zad1 = return_rng();
    println!("Tablica po uzupelnieniu losowymi liczbami: {:?}",tab_zad1);
    let mut min = 101;
    
    for i in tab_zad1{
        if min > i {
            min = i;
        }
    }
    println!("To jest najmniejsza wartosc: {min}");
    println!("Zlozonosc programu: O(n)");


    // zadanie 3
    println!("\nZadanie 3");
    let tab_zad3 = return_rng();
    println!("Tablica: {:?}", tab_zad3);
    let mut min = 101;
    let mut min_ind = 100;

    for i in 0..10 {
        if tab_zad3[i] < min {
            min = tab_zad3[i];
            min_ind = i;
        }
    }
    println!("Minimalny index: {min_ind}\nNajmniejsza liczba: {min}");

    // zadanie 4
    println!("\nZadanie 4");
    let tab_zad4 = return_rng();
    // let tab_zad4 = vec![10,12,12,12,12,12,12,12,12,10];
    println!("Tablica: {:?}", tab_zad4);
    let mut min = 101;
    let mut min_ind = 100;

    for i in 0..10 {
        if tab_zad4[i] <= min {
            min = tab_zad4[i];
            min_ind = i;
        }
    }
    println!("Maksymalny index: {min_ind}\nNajmniejsza liczba: {min}");

    // zadanie 6
    println!("\nZadanie 6");
    let tab_zad6 = return_rng();
    println!("Tablica: {:?}", tab_zad6);
    let mut min = 101;
    let mut max = 0;

    for i in tab_zad6 {
        if i > max {max = i;}
        if i < min {min = i;}
    }
    println!("Wartosc maksymalna: {max}\nWartosc minimalna: {min}");

    // zadnie 7
    println!("\nZadanie 7");
    let tab_zad7 = return_rng();
    println!("Tablica: {:?}", tab_zad7.clone());
    println!("Suma wszystkich elementow w tablicy: {}", zadanie_7a(tab_zad7.clone()));
    println!("Iloczyn wszytkich elementow w tablicy: {}", zadanie_7b(tab_zad7.clone()));
    println!("Srednia arytmetyczna elementow w tablicy: {}", zadanie_7c(tab_zad7.clone()));
    println!("Suma iloczynow w jakis forze: {}", zadanie_7e(vec![2,3,4,5,6,7]));

    // zadanie 8
    println!("\nZadanie 8");
    let mut tab_zad8 = return_rng();
    tab_zad8.sort();
    println!("Tablica {:?}", tab_zad8);
    let mut rng = rand::thread_rng();
    let n: u64 = rng.gen_range(0..100);
    println!("{}", zadanie_8(tab_zad8.clone(), n));

    // zadanie 9
    println!("\nZadanie 9");
    let mut tab_zad9_a = return_rng();
    tab_zad9_a.sort();
    let mut tab_zad9_b = return_rng();
    tab_zad9_b.sort();
    println!("Tablica A: {:?}\nTablica B: {:?}", tab_zad9_a, tab_zad9_b);

    let x = 130;
    println!("{}", zadanie_9(tab_zad9_a.clone(), tab_zad9_b.clone(), x));

}
