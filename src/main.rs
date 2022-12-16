use crate::exact_sciences::*;
use crate::human_sciences::*;

fn main() {
exact_sciences::math();

}
mod exact_sciences {

    pub fn math() {
        let nota_math1: u8 = 10;
        let nota_math2: u8 = 5;
        let nota_math3: u8 = 2;
        let nota_math4: u8 = 8;
        let media_math: u8 =  (nota_math1 + nota_math2 + nota_math3 + nota_math4) / 4;
    }

    pub fn physical() {
        let nota_physical1: u8 = 10;
        let nota_physical2: u8 = 8;
        let nota_physical3: u8 = 9;
        let nota_physical4: u8 = 6;
        let media_physical: u8 =  (nota_physical1 + nota_physical2 + nota_physical3 + nota_physical4) / 4;
    }

    pub fn chemistry() {
        let nota_chemistry1: u8 = 6;
        let nota_chemistry2: u8 = 7;
        let nota_chemistry3: u8 = 4;
        let nota_chemistry4: u8 = 8;
        let media_chemistry: u8 =  (nota_chemistry1 + nota_chemistry2 + nota_chemistry3 + nota_chemistry4) / 4;
    }
}

mod human_sciences {

    pub fn portuguese() {
        let nota_portuguese1: u8 = 9;
        let nota_portuguese2: u8 = 10;
        let nota_portuguese3: u8 = 10;
        let nota_portuguese4: u8 = 8;
        let media_portuguese: u8 =  (nota_portuguese1 + nota_portuguese2 + nota_portuguese3 + nota_portuguese4) / 4;
    }

    pub fn english() {
        let nota_english1: u8 = 10;
        let nota_english2: u8 = 7;
        let nota_english3: u8 = 9;
        let nota_english4: u8 = 10;
        let media_english: u8 =  (nota_english1 + nota_english2 + nota_english3 + nota_english4) / 4;
    }

    pub fn history() {
        let nota_history1: u8 = 10;
        let nota_history2: u8 = 10;
        let nota_history3: u8 = 9;
        let nota_history4: u8 = 8;
        let media_history: u8 =  (nota_history1 + nota_history2 + nota_history3 + nota_history4) / 4;
    }

    pub fn geography() {
        let nota_geography1: u8 = 7;
        let nota_geography2: u8 = 6;
        let nota_geography3: u8 = 7;
        let nota_geography4: u8 = 8;
        let media_geography: u8 =  (nota_geography1 + nota_geography2 + nota_geography3 + nota_geography4) / 4;
    }
}