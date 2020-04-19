use std::{
    fmt::{self, Debug, Formatter},
    mem::swap,
};

pub trait Tube: Default + Debug {
    fn size() -> usize;
    fn empty(&self) -> bool;
    fn sorted(&self) -> bool;
    fn full(&self) -> bool;
    fn last(&self) -> u8;
    fn pop(&mut self) -> u8;
    fn push(&mut self, other: u8);
    fn can_move<T: Tube>(&mut self, other: &mut T) -> bool {
        !self.empty() && !other.full() && self.last() == other.last()
    }
    fn move_to<T: Tube>(&mut self, other: &mut T) {
        assert!(!self.can_move(other));
        other.push(self.pop())
    }
}

#[derive(Default)]
pub struct Tube4(u8, u8, u8, u8);
// pub struct Tube4(u32);
// impl Tube4 {
// pub fn encode(s: (u8, u8, u8, u8)) -> Self {
// unsafe { Self(transmute::<[u8; 4], u32>([s.0, s.1, s.2, s.3])) }
// }
//
// pub fn decode(&self) -> (u8, u8, u8, u8) {
// let o = unsafe { transmute::<u32, [u8; 4]>(self.0) };
// return (o[0], o[1], o[2], o[3]);
// }
// }

impl Tube for Tube4 {
    fn size() -> usize {
        return 4;
    }

    fn empty(&self) -> bool {
        if self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0 { true } else { false }
    }

    fn sorted(&self) -> bool {
        let a = self.0;
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

    fn full(&self) -> bool {
        self.3 != 0
    }

    // Crazy hardcoded
    fn last(&self) -> u8 {
        assert!(!self.empty());
        if self.3 != 0 {
            return self.3;
        }
        else if self.2 != 0 {
            return self.2;
        }
        else if self.1 != 0 {
            return self.1;
        }
        else if self.1 != 0 {
            return self.0;
        }
        unreachable!()
    }

    // Crazy hardcoded
    fn pop(&mut self) -> u8 {
        assert!(!self.empty());
        let mut out = 0;
        if self.3 != 0 {
            swap(&mut self.3, &mut out);
            return out;
        }
        else if self.2 != 0 {
            swap(&mut self.2, &mut out);
            return out;
        }
        else if self.1 != 0 {
            swap(&mut self.1, &mut out);
            return out;
        }
        else if self.0 != 0 {
            swap(&mut self.0, &mut out);
            return out;
        }
        unreachable!()
    }

    // Crazy hardcoded
    fn push(&mut self, other: u8) {
        assert!(!self.full());
        if self.0 == 0 {
            self.0 = other;
            return;
        }
        else if self.1 == 0 {
            self.1 = other;
            return;
        }
        else if self.2 == 0 {
            self.2 = other;
            return;
        }
        else if self.3 == 0 {
            self.3 = other;
            return;
        }
        unreachable!()
    }
}

impl Debug for Tube4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // f.debug_tuple("Tube").field(&self.0).field(&self.1).field(&self.2).field(&self.3).finish()
        write!(f, "Tube({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}
