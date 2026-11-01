#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Dash,
    LongJump,
    Hurdles,
    PoleVault,
    Javelin,
}
impl Event {
    pub const ALL: [Self; 5] = [
        Self::Dash,
        Self::LongJump,
        Self::Hurdles,
        Self::PoleVault,
        Self::Javelin,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Dash => "100m Dash",
            Self::LongJump => "Long Jump",
            Self::Hurdles => "110m Hurdles",
            Self::PoleVault => "Pole Vault",
            Self::Javelin => "Javelin",
        }
    }
    pub fn race(self) -> bool {
        matches!(self, Self::Dash | Self::Hurdles)
    }
    pub fn instructions(self) -> &'static str {
        match self {
            Self::Dash => "Follow the blue 4 / green 6 prompts. Press 5 near the finish to dive.",
            Self::LongJump => {
                "Follow 4 / 6. Press 5 before the board, then 5 at the apex to stretch."
            }
            Self::Hurdles => {
                "Complete each three-button 4 / 6 sequence. Hurdle jumps are automatic."
            }
            Self::PoleVault => {
                "Follow 4 / 6. Hold 5 as the pole tip reaches the box; release as it straightens."
            }
            Self::Javelin => {
                "Follow 4 / 6. Hold 5 to charge; release when the marker reaches the tip."
            }
        }
    }
    pub fn finish_message(self, result: Option<i32>) -> &'static str {
        let points = self.score(result).max(0);
        [
            "AWESOME!",
            "GREAT!",
            "NICE!",
            "ALMOST THERE",
            "NOT GOOD ENOUGH",
            "OUCH",
        ][[900, 750, 600, 450, 300]
            .iter()
            .position(|threshold| points >= *threshold)
            .unwrap_or(5)]
    }
    pub fn score(self, result: Option<i32>) -> i32 {
        let Some(n) = result else { return 0 };
        let lerp = |a: i32, b: i32, x: i32, d: i32| (a * (d - x) + b * x) / d;
        match self {
            Self::Dash => {
                if n < 1120 {
                    lerp(600, 1000, 1120 - n, 146)
                } else {
                    lerp(600, 0, n - 1120, 380)
                }
            }
            Self::Hurdles => {
                if n < 1450 {
                    lerp(600, 1000, 1450 - n, 162)
                } else {
                    lerp(600, 0, n - 1450, 270)
                }
            }
            Self::LongJump => {
                if n > 550 {
                    lerp(600, 1000, n - 550, 345)
                } else {
                    lerp(600, 0, 550 - n, 250)
                }
            }
            Self::PoleVault => {
                if n > 500 {
                    lerp(600, 1000, n - 500, 114)
                } else {
                    lerp(600, 0, 500 - n, 300)
                }
            }
            Self::Javelin => {
                if n > 6500 {
                    lerp(600, 1000, n - 6500, 3348)
                } else {
                    lerp(600, 0, 6500 - n, 4500)
                }
            }
        }
        .max(0)
    }
    pub fn result(self, n: Option<i32>) -> String {
        n.map_or("FOUL".into(), |v| {
            format!(
                "{}.{:02} {}",
                v / 100,
                v % 100,
                if self.race() { "s" } else { "m" }
            )
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Prepare,
    LandGood,
    LandBad,
    FoulRun,
    Run,
    Approach,
    Air,
    Stretch,
    BadStretch,
    Charge,
    Throw,
    Flight,
    VaultPlant,
    VaultBend,
    VaultStraighten,
    VaultFall,
    VaultRelease,
    Done,
}
