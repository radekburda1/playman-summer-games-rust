#[derive(Clone, Debug)]
pub struct Runner {
    pub position: i32,
    pub speed: i32,
    pub height: i32,
    pub vertical_velocity: i32,
    pub expected_key: usize,
    previous: usize,
    rng: u64,
    ages: [i32; 10],
    pub sequence: [usize; 3],
    pub sequence_cursor: usize,
    pub mistakes: i32,
    pub previous_mistakes: i32,
    pub tick: i32,
    last_press: i32,
    burst: i32,
    pub dive: Option<i32>,
}
impl Runner {
    pub fn new(seed: u64, pos: i32) -> Self {
        Self {
            position: pos,
            speed: -1,
            height: 0,
            vertical_velocity: 0,
            expected_key: ((seed >> 6) & 1) as usize,
            previous: ((seed >> 4) & 1) as usize,
            rng: (seed ^ 0x5DEECE66D) & ((1 << 48) - 1),
            ages: [22; 10],
            sequence: [
                ((seed >> 4) & 1) as usize,
                ((seed >> 6) & 1) as usize,
                ((seed >> 7) & 1) as usize,
            ],
            sequence_cursor: 0,
            mistakes: 0,
            previous_mistakes: 0,
            tick: 0,
            last_press: 0,
            burst: 0,
            dive: None,
        }
    }
    fn randomize(&mut self) {
        self.rng = (self.rng.wrapping_mul(25214903917) + 11) & ((1 << 48) - 1);
    }
    pub fn next_sequence(&mut self) {
        self.randomize();
        self.sequence = [
            ((self.rng >> 4) & 1) as usize,
            ((self.rng >> 6) & 1) as usize,
            ((self.rng >> 7) & 1) as usize,
        ];
        self.sequence_cursor = 0;
        self.penalty(50 * self.mistakes / 3);
        self.speed = 24000 - 8000 * (self.ages.iter().sum::<i32>() - 80) / 140;
        self.previous_mistakes = self.mistakes;
        self.mistakes = 0;
    }
    fn penalty(&mut self, mut amount: i32) {
        for a in self.ages.iter_mut().rev() {
            let n = (22 - *a).min(amount);
            *a += n;
            amount -= n;
            if amount <= 0 {
                break;
            }
        }
    }
    pub fn press(&mut self, key: usize, hurdles: bool) -> bool {
        if self.speed < 0 {
            self.speed = 0
        }
        if self.tick - self.last_press <= 4 {
            self.burst = (self.burst + 1).min(12)
        } else {
            self.burst -= 1
        }
        self.last_press = self.tick;
        if self.dive.is_some() || hurdles && self.sequence_cursor >= 3 {
            return false;
        }
        let correct = key
            == if hurdles {
                self.sequence[self.sequence_cursor]
            } else {
                self.expected_key
            };
        if correct {
            self.ages.rotate_right(1);
            self.ages[0] = 0;
        } else if hurdles {
            self.mistakes += 1
        } else {
            self.penalty(50)
        }
        if hurdles {
            self.sequence_cursor += 1
        } else if correct {
            if self.expected_key == self.previous {
                self.expected_key = 1 - self.expected_key
            } else {
                self.previous = self.expected_key;
                self.randomize();
                self.expected_key = ((self.rng >> 5) & 1) as usize
            }
        }
        correct
    }
    pub fn run(&mut self, hurdles: bool) {
        if self.burst >= 6 && self.tick - self.last_press > (self.burst - 5) * 4 {
            self.burst = 0
        }
        if self.speed >= 0 {
            let mut budget = if self.burst >= 6 { 30 } else { 10 };
            while budget > 0 {
                if self.ages[0] < 22 {
                    self.ages[0] += 1;
                    budget -= 10
                } else {
                    self.ages.rotate_right(1)
                }
                budget -= 1;
            }
            let sum: i32 = self.ages.iter().sum();
            let base = if hurdles { 4 } else { 8 };
            self.speed = 24000 - 8000 * (sum - 10 * base) / (10 * (22 - base));
            self.move_by(0);
        }
    }
    pub fn move_by(&mut self, gravity: i32) {
        if self.speed >= 0 {
            if let Some(start) = self.dive {
                self.position += if self.position - start < 409600 {
                    3900
                } else {
                    -3900
                }
            }
            self.position += self.speed;
            self.vertical_velocity -= gravity;
            self.height += self.vertical_velocity;
        }
    }
}
