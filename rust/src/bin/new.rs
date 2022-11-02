struct Test {
    pub a: int,
}

pub fn funkcija_ne_metoda(ime: String) {}

impl Test {
    fn staticna_metoda(ime: String) {}
    fn navadna_metoda(self, ime: String) {
        self.a
    }
}

fn main() {
    funkcija_ne_metoda(ime);
    let test = Test { a: 3 };

    test.navadna_metoda(ime);
    Test::navadna_metoda(test, ime);
    Test::staticna_metoda(test, "a");
}
