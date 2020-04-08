trait Tube {
    fn empty(&self) -> bool;
    fn sorted(&self) -> bool;
}

#[derive(Default, Debug)]
struct Tube4(u8, u8, u8, u8);

impl Tube for Tube4 {
    fn empty(&self) -> bool {
        if self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0 { true } else { false }
    }

    fn sorted(&self) -> bool {
        let a = &self.0;
        if a == 0 {
            false
        }
        else if self.1 == a && self.2 == a && self.3 == a {
            true
        }
        else {
            false
        }
    }
}
