use rand::{rngs::ThreadRng, Rng};

const CHAR5W: [i32; 90] = [
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    60, 660, 1260, 1860, 2460, 3060, 3660, 4260, 4860, 5460, 6060, 6660, 7260, 7860, 8460, 9060,
    9660, 10260,
];
const CHAR4W: [i32; 10] = [510, 510, 510, 510, 510, 510, 510, 510, 5610, 10710];
const WEAP5W: [i32; 80] = [
    70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 770, 1470, 2170, 2870, 3570, 4270,
    4970, 5670, 6370, 7070, 7770, 8470, 9170, 9870, 10570, 11270, 11970, 12670,
];
const WEAP4W: [i32; 10] = [600, 600, 600, 600, 600, 600, 600, 6600, 12600, 18600];

#[derive(Debug, Default)]
pub struct Gacha {
    // records
    pulls: usize,     // intertwined fates
    stars: usize,     // starglitters
    char5up: usize,   // 5 star character up
    char5down: usize, // 5 star character down
    weap5up: usize,   // 5 star weapon up
    weap5down: usize, // 5 star weapon down
    // states
    rng: ThreadRng,
    char5pity: usize, // 5 star character pity
    char5guar: usize, // 5 star up character guarantees
    char4pity: usize, // 4 star character pity
    weap5pity: usize, // 5 star weapon pity
    weap5guar: usize, // 5 star up weapon guarantees
    weap4pity: usize, // 4 star weapon pity
}

impl Gacha {
    pub fn gacha_char(mut self) -> Self {
        self.pulls = self.pulls + 1;
        if self.rng.gen_range(1..=10000) < CHAR5W[self.char5pity] {
            (self.stars, self.char5pity, self.char4pity) = (self.stars + 5, 0, self.char4pity + 1);
            match (self.char5guar, self.rng.gen_range(1..=10000)) {
                (0, 1..=5000) | (1, _) => {
                    (self.char5up, self.char5down, self.char5guar) =
                        (self.char5up + 1, self.char5down + 0, 0);
                }
                (_, _) => {
                    (self.char5up, self.char5down, self.char5guar) =
                        (self.char5up + 0, self.char5down + 1, self.char5guar + 1);
                }
            }
        } else if self.char4pity >= 10 || self.rng.gen_range(1..=10000) < CHAR4W[self.char4pity] {
            (self.stars, self.char5pity, self.char4pity) = (self.stars + 2, self.char5pity + 1, 0);
        } else {
            (self.stars, self.char5pity, self.char4pity) =
                (self.stars + 0, self.char5pity + 1, self.char4pity + 1);
        }
        self
    }

    pub fn gacha_weap(mut self) -> Self {
        self.pulls = self.pulls + 1;
        if self.rng.gen_range(1..=10000) < WEAP5W[self.weap5pity] {
            (self.stars, self.weap5pity, self.weap4pity) = (self.stars + 5, 0, self.weap4pity + 1);
            match (self.weap5guar, self.rng.gen_range(1..=10000)) {
                (0, 1..=3750) | (1, 1..=5000) | (2, _) => {
                    (self.weap5up, self.weap5down, self.weap5guar) =
                        (self.weap5up + 1, self.weap5down + 0, 0);
                }
                (_, _) => {
                    (self.weap5up, self.weap5down, self.weap5guar) =
                        (self.weap5up + 0, self.weap5down + 1, self.weap5guar + 1);
                }
            }
        } else if self.weap4pity >= 10 || self.rng.gen_range(1..=10000) < WEAP4W[self.weap4pity] {
            (self.stars, self.weap5pity, self.weap4pity) = (self.stars + 2, self.weap5pity + 1, 0);
        } else {
            (self.stars, self.weap5pity, self.weap4pity) =
                (self.stars + 0, self.weap5pity + 1, self.weap4pity + 1);
        }
        self
    }

    pub fn gacha_char5up(mut self, num: usize) -> Self {
        let char5up = self.char5up;
        loop {
            self = self.gacha_char();
            if char5up + num == self.char5up {
                break;
            }
        }
        self
    }

    pub fn gacha_char_n(mut self, num: usize) -> Self {
        for _ in 1..=num {
            self = self.gacha_char();
        }
        self
    }

    pub fn gacha_weap5up(mut self, num: usize) -> Self {
        let weap5up = self.weap5up;
        loop {
            self = self.gacha_weap();
            if weap5up + num == self.weap5up {
                break;
            }
        }
        self
    }

    pub fn gacha_weap_n(mut self, num: usize) -> Self {
        for _ in 1..=num {
            self = self.gacha_weap();
        }
        self
    }
}

fn main() {
    let gacha = Gacha {
        char5pity: 58,
        weap5pity: 15,
        ..Default::default()
    }
    .gacha_char5up(3)
    .gacha_weap5up(1);
    println!("{gacha:?}");
}
