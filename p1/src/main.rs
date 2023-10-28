use std::{f32::consts::PI};

fn main(){
    // Zadanie 1
    println!("Zadanie 1");
    let mut t = 0.54;
    let mut v0 = 3.44;
    let g = 10.0;
    println!("Dla t = {}, wysokosc wynosi: {}", t, v0*t-(1.0/2.0)*g*t*t);
    t = 0.1;
    println!("Dla t = {}, wysokosc wynosi: {}", t, v0*t-(1.0/2.0)*g*t*t);
    t = 0.235;
    println!("Dla t = {}, wysokosc wynosi: {}", t, v0*t-(1.0/2.0)*g*t*t);
    
    // Zadanie 2
    println!("\nW zadaniu 2 przyjmujemy t = 1s");
    t = 1.0;
    let mut gPlanety = 3.7;
    println!("Na Merkurym i Marsie, wysokosc wynosi: {}", v0*t-(1.0/2.0)*gPlanety*t*t);
    gPlanety = 8.9;
    println!("Na Wenus, wysokosc wynosi: {}", v0*t-(1.0/2.0)*gPlanety*t*t);
    gPlanety = 23.1;
    println!("Na Jowisz, wysokosc wynosi: {}", v0*t-(1.0/2.0)*gPlanety*t*t);
    gPlanety = 9.0;
    println!("Na Saturnie, wysokosc wynosi: {}", v0*t-(1.0/2.0)*gPlanety*t*t);
    gPlanety = 8.7;
    println!("Na Uranie, wysokosc wynosi: {}", v0*t-(1.0/2.0)*gPlanety*t*t);
    gPlanety = 11.0;
    println!("Na Neptunie, wysokosc wynosi: {}", v0*t-(1.0/2.0)*gPlanety*t*t);

    // Zadanie 3
    println!("\nZadanie 3");
    let metry = 640.;
    let cal = metry * 100.0 / 2.54;
    let stopa = cal / 12.;
    let jard = stopa / 3.;
    let mila = jard / 1760.;
    println!("Metry: {}\nCale: {}\nStopy: {}\nJardy: {}\nMile: {}\n", metry, cal, stopa, jard, mila);

    // Zadanie 4
    println!("\nZadanie 4");
    let celsius = 1.;
    let fahren = 9./5. * celsius + 32.;
    let to_celsius = (fahren - 32.) * (5./9.);
    println!("Stopnie Celsjusza: {}\nZamiana na Fahrenheity: {}\nPowrot na Celsjusza: {}\n", celsius, fahren, to_celsius);

    // Zadanie 5
    println!("\nZadanie 5");
    let promien = 1.;
    let pole = PI * promien * promien;
    let obwod = 2. * PI * promien;
    println!("Promien kola: {}\nPole kola: {}\nObwod kola: {}\n", promien, pole, obwod);

    // Zadanie 6
    println!("\nZadanie 6");
    let mut alpha = 180.;
    let polew = pole * alpha / 360.;
    let obwodw = obwod * alpha / 360.;
    println!("Promien kola: {}\nAlpha: {}\nPole wycinka: {}\nObwod wycinka: {}", promien, alpha, polew, obwodw);

    // Zadanie 7
    println!("\nZadanie 7");
    let h0 = 12.5;
    let mut x = 2.963;
    v0 = 3.;
    alpha = 0.12;
    let cos2 = alpha.cos() * alpha.cos();
    let tg = alpha.tan();
    let y = h0 + x*tg - (g*x*x) / (2. * v0 * v0 * cos2);
    println!("W odległości {x} od miejsca wyrzutu ciało rzucone z wysokości {h0} m z prędkością początkową {v0} m/s pod kątem {alpha} rad znajduje się na wysokości {y}  m w polu grawitacyjnym Ziemi.");

    // Zadanie 8
    println!("\nZadanie 8");
    let k0 = 1000.;
    let mut m = 1.;
    let mut n = 2.;
    let mut r = 0.05;
    let mut K = k0 * f32::powf(1.+r/m, m*n);
    println!("Stopa procentowa: {r}\nLiczba okresow kapitalizacji: {m}\nLokata: {n}\nKwota wplacona: {k0}\nKwota koncowa: {K}\n");
    m = 4.;
    n = 3.;
    r = 0.023;
    K = k0 * f32::powf(1.+r/m, m*n);
    println!("Stopa procentowa: {r}\nLiczba okresow kapitalizacji: {m}\nLokata: {n}\nKwota wplacona: {k0}\nKwota koncowa: {K}\n");

    // Zadanie 9
    println!("\nZadanie 9");
    println!("{:e}\n{:e}\n{:e}\n{:2e}\nOdleglosc od Slonca: {:e}km\n", 12345, 0.12345, 0.00000123, 1020304050, 146900000);

    // Zadanie 10
    println!("\nZadanie 10");
    println!("Ziemia w ciagu godziny pokona: {:e} km\nW ciagu jednego dnia: {:e} km\nW ciagu jednego roku: {:e} km", 30 * 3600, 30 * 3600 * 24,  30 * 3600 * 24 * 365);
}

