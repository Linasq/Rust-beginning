#[derive(Debug)]
enum Smak {
    Slodki,
    Gorzki,
    Kwasny
}
struct Gruszka {
    odmiana: String,
    kolor: String,
    waga: f32,
    wielkosc: f32,
    smak: Smak
}

impl Gruszka {  
    fn new(odmiana: String, kolor: String, wielkosc: f32, waga: f32, smak: Smak) -> Self{
        Gruszka{
            odmiana: odmiana,
            kolor: kolor,
            waga: waga,
            wielkosc: wielkosc,
            smak: smak
        }
    }
    
   fn get_odmiana(&self) -> String {
        self.odmiana.clone()
   }
   
   fn get_kolor(&self) -> String {
        self.kolor.clone()
   }

   fn get_waga(&self) -> f32 {
        self.waga
   }

   fn get_wielkosc(&self) -> f32 {
        self.wielkosc
   }

   fn get_smak(&self) -> &Smak{
       &self.smak
   }
}

/////////////////////////////////////////////////////////////////////////////////////
struct Motor{
    marka: String,
    pojemnosc: f32,
    KM: i16
}

impl Motor {
    fn new(marka: String, pojemnosc: f32, KM: i16) -> Self {
        Motor { marka: marka,  pojemnosc: pojemnosc, KM: KM }
    }

    fn get_marka(&self) -> String {self.marka.clone()}
    fn get_pojemnosc(&self) -> f32 {self.pojemnosc}
    fn get_KM(&self) -> i16 {self.KM}
}


//////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

struct Rakieta{
    waga: f32,
    paliwo: f32,
    polozenie: (f32, f32),
    predkosc: f32,
    kierunek: Direction
}

impl Rakieta {
    fn new(waga: f32, paliwo: f32, x: f32, y: f32, predkosc: f32, kierunek: Direction) -> Self {
       Rakieta { waga: waga+paliwo*1.5, paliwo: paliwo, polozenie: (x,y), predkosc: predkosc, kierunek: kierunek } 
    }

    fn get_waga(&self) -> f32 {self.waga}
    fn get_paliwo(&self) -> f32 {self.paliwo}
    fn get_polozenie(&self) -> (f32, f32) {self.polozenie}
    fn get_predkosc(&self) -> f32 {self.predkosc}
    fn get_kierunek(&self) -> &Direction {&self.kierunek}

   fn lot(&mut self, czas_lotu: f32) {
        match self.kierunek {
            Direction::NORTH => {self.polozenie.1 += self.predkosc * czas_lotu;}
            Direction::SOUTH => {self.polozenie.1 -= self.predkosc * czas_lotu;}
            Direction::EAST => {self.polozenie.0 += self.predkosc * czas_lotu;}
            Direction::WEST => {self.polozenie.0 -= self.predkosc * czas_lotu;}
        }
        let straconePaliwo = 2. * czas_lotu;
        self.paliwo -= straconePaliwo;
        self.waga -= straconePaliwo*1.5;
   } 
}

//////////////////////////////////////////////////////////////////////////////////////
fn main() {
    // struct Gruszki
    println!("\nGruszka");
    let superGruszka = Gruszka::new("lesna".to_string(), "zielona".to_string(), 10.2, 0.23, Smak::Slodki);

    println!("Odmiana: {}\nKolor: {}\nWielkosc: {}\nWaga: {}\nSmak: {:?}", superGruszka.get_odmiana(), superGruszka.get_kolor(), superGruszka.get_wielkosc(), superGruszka.get_waga(), superGruszka.get_smak());


    // struct Motor
    println!("\nMotocykl");
    let motocykl = Motor::new("Kawasaki".to_string(), 0.7, 100);
    println!("Model: {}\nPojemnosc: {}\nKonie mechaniczne: {}", motocykl.get_marka(), motocykl.get_pojemnosc(), motocykl.get_KM());


    // struct rakieta
    println!("\nRakieta");

    let mut rakieta = Rakieta::new(1000., 100., 2., 0., 2., Direction::NORTH);
    println!("\nStan poczatkowy rakiety:\nwaga: {}\npaliwo: {}\npolozenie: {:?}\npredkosc: {}\nkierunek: {:?}", rakieta.get_waga(), rakieta.get_paliwo(), rakieta.get_polozenie(), rakieta.get_predkosc(), rakieta.get_kierunek());

    rakieta.lot(2.);
    println!("\nStan po locie przez 1s:\nwaga: {}\npaliwo: {}\npolozenie: {:?}\npredkosc: {}\nkierunek: {:?}", rakieta.get_waga(), rakieta.get_paliwo(), rakieta.get_polozenie(), rakieta.get_predkosc(), rakieta.get_kierunek());
}
