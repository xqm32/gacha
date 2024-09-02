use clap::Parser;
use rand::{rngs::ThreadRng, Rng};

const U5C_W: [i32; 90] = [
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
    60, 660, 1260, 1860, 2460, 3060, 3660, 4260, 4860, 5460, 6060, 6660, 7260, 7860, 8460, 9060,
    9660, 10260,
];
const U4C_W: [i32; 10] = [510, 510, 510, 510, 510, 510, 510, 510, 5610, 10710];
const U5W_W: [i32; 80] = [
    70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 770, 1470, 2170, 2870, 3570, 4270,
    4970, 5670, 6370, 7070, 7770, 8470, 9170, 9870, 10570, 11270, 11970, 12670,
];
const U4W_W: [i32; 10] = [600, 600, 600, 600, 600, 600, 600, 6600, 12600, 18600];

#[derive(Debug, Default, Clone, Copy)]
pub struct Gacha {
    // records
    pub pulls: usize,      // Intertwined fates
    pub stars: usize,      // Starglitters
    pub chars_up: usize,   // 5 star characters up
    pub chars_down: usize, // 5 star characters down
    pub weaps_up: usize,   // 5 star weapons up
    pub weaps_down: usize, // 5 star weapons down
    // states
    pub u5c_pity: usize, // Up 5 star character pity
    pub u5c_guar: usize, // Up 5 star up character guarantees
    pub u4c_pity: usize, // Up 4 star character pity
    pub u5w_pity: usize, // Up 5 star weapon pity
    pub u5w_guar: usize, // Up 5 star up weapon guarantees
    pub u4w_pity: usize, // Up 4 star weapon pity
    // events
    pub on_char_up: Option<fn(&Gacha) -> ()>, // 5 star character up event
    pub on_char_down: Option<fn(&Gacha) -> ()>, // 5 star character down event
    pub on_char_light: Option<fn(&Gacha) -> ()>, // 5 star character light event
    pub on_weap_up: Option<fn(&Gacha) -> ()>, // 5 star weapon up event
    pub on_another_weap_up: Option<fn(&Gacha) -> ()>, // Another 5 star weapon up event
    pub on_weap_down: Option<fn(&Gacha) -> ()>, // 5 star weapon down event
}

impl Gacha {
    pub fn pull_char(mut self) -> Self {
        let mut rng = ThreadRng::default();
        (self.pulls, self.u5c_pity, self.u4c_pity) =
            (self.pulls + 1, self.u5c_pity + 1, self.u4c_pity + 1);
        if rng.gen_range(1..=10000) <= U5C_W[self.u5c_pity - 1] {
            match (self.u5c_guar, rng.gen_range(1..=10000)) {
                (0, 1..=5000) | (1, _) => {
                    if let Some(on_event) = self.on_char_up {
                        on_event(&self)
                    }
                    (self.chars_up, self.u5c_guar) = (self.chars_up + 1, 0);
                }
                (0, 5001..=5500) => {
                    if let Some(on_event) = self.on_char_light {
                        on_event(&self)
                    }
                    (self.chars_up, self.u5c_guar) = (self.chars_up + 1, self.u5c_guar + 1);
                }
                (_, _) => {
                    if let Some(on_event) = self.on_char_down {
                        on_event(&self)
                    }
                    (self.chars_down, self.u5c_guar) = (self.chars_down + 1, self.u5c_guar + 1);
                }
            }
            (self.stars, self.u5c_pity) = (self.stars + 5, 0);
        } else if self.u4c_pity >= 10 || rng.gen_range(1..=10000) <= U4C_W[self.u4c_pity - 1] {
            (self.stars, self.u4c_pity) = (self.stars + 2, 0);
        }
        self
    }

    pub fn pull_weap(mut self) -> Self {
        let mut rng = ThreadRng::default();
        (self.pulls, self.u5w_pity, self.u4w_pity) =
            (self.pulls + 1, self.u5w_pity + 1, self.u4w_pity + 1);
        if rng.gen_range(1..=10000) <= U5W_W[self.u5w_pity - 1] {
            match (self.u5w_guar, rng.gen_range(1..=10000)) {
                (0, 1..=3750) | (1, _) => {
                    if let Some(on_event) = self.on_weap_up {
                        on_event(&self)
                    }
                    (self.weaps_up, self.u5w_guar) = (self.weaps_up + 1, 0);
                }
                (0, 3751..=7500) => {
                    if let Some(on_event) = self.on_another_weap_up {
                        on_event(&self)
                    }
                    (self.weaps_down, self.u5w_guar) = (self.weaps_down + 1, self.u5w_guar + 1);
                }
                _ => {
                    if let Some(on_event) = self.on_weap_down {
                        on_event(&self)
                    }
                    (self.weaps_down, self.u5w_guar) = (self.weaps_down + 1, self.u5w_guar + 1);
                }
            }
            (self.stars, self.u5w_pity) = (self.stars + 5, 0);
        } else if self.u4w_pity >= 10 || rng.gen_range(1..=10000) <= U4W_W[self.u4w_pity - 1] {
            (self.stars, self.u4w_pity) = (self.stars + 2, 0);
        }
        self
    }
}

impl Gacha {
    pub fn with_pity(self, char_pity: usize, weap_pity: usize) -> Self {
        Gacha {
            u5c_pity: char_pity,
            u5w_pity: weap_pity,
            ..self
        }
    }

    pub fn with_guar(self, char_guar: usize, weap_guar: usize) -> Self {
        Gacha {
            u5c_guar: char_guar,
            u5w_guar: weap_guar,
            ..self
        }
    }

    pub fn pull_chars(mut self, mut pulls: usize) -> Self {
        pulls += self.pulls;
        while self.pulls < pulls {
            self = self.pull_char();
        }
        self
    }

    pub fn pull_weaps(mut self, mut pulls: usize) -> Self {
        pulls += self.pulls;
        while self.pulls < pulls {
            self = self.pull_weap();
        }
        self
    }

    pub fn pull_chars_up(mut self, mut ups: usize) -> Self {
        ups += self.chars_up;
        while self.chars_up < ups {
            self = self.pull_char();
        }
        self
    }

    pub fn pull_weaps_up(mut self, mut ups: usize) -> Self {
        ups += self.weaps_up;
        while self.weaps_up < ups {
            self = self.pull_weap();
        }
        self
    }

    pub fn pull_up(self, chars_up: usize, weaps_up: usize) -> Self {
        self.pull_chars_up(chars_up).pull_weaps_up(weaps_up)
    }
}

/// Genshin Impact Gacha Simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gacha times
    #[arg(short, long, default_value_t = 1)]
    times: usize,
    /// 5 star characters up
    #[arg(short, long, default_value_t = 1)]
    chars_up: usize,
    /// 5 star character pity
    #[arg(short = 'C', long, default_value_t = 0)]
    char_pity: usize,
    /// 5 star weapons up
    #[arg(short, long, default_value_t = 0)]
    weaps_up: usize,
    /// 5 star weapon pity
    #[arg(short = 'W', long, default_value_t = 0)]
    weap_pity: usize,
    /// Verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut gacha = Gacha::default().with_pity(args.char_pity, args.weap_pity);
    if args.verbose {
        gacha.on_char_up = Some(|gacha: &Gacha| {
            println!("  UP CHAR {:4} {:4}", gacha.pulls, gacha.u5c_pity);
        });
        gacha.on_char_down = Some(|gacha: &Gacha| {
            println!("DOWN CHAR {:4} {:4}", gacha.pulls, gacha.u5c_pity);
        });
        gacha.on_char_light = Some(|gacha: &Gacha| {
            println!("LIGH CHAR {:4} {:4}", gacha.pulls, gacha.u5c_pity);
        });
        gacha.on_weap_up = Some(|gacha: &Gacha| {
            println!("  UP WEAP {:4} {:4}", gacha.pulls, gacha.u5w_pity);
        });
        gacha.on_another_weap_up = Some(|gacha: &Gacha| {
            println!("ANOT WEAP {:4} {:4}", gacha.pulls, gacha.u5w_pity);
        });
        gacha.on_weap_down = Some(|gacha: &Gacha| {
            println!("DOWN WEAP {:4} {:4}", gacha.pulls, gacha.u5w_pity);
        });
    }

    let mut sum = 0;
    for _ in 1..=args.times {
        sum += gacha.pull_up(args.chars_up, args.weaps_up).pulls;
        if args.verbose {
            println!();
        }
    }
    println!("{:.2}", sum as f64 / args.times as f64);
}
