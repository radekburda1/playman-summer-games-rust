//! Logical audio requests. Playback and decoding belong to the platform adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum SoundCue {
    Record = 8,
    RaceFinish = 13,
    Takeoff = 18,
    WrongInput = 19,
    Action = 20,
    Failure = 22,
    Ready = 23,
    Set = 24,
    Gun = 25,
    Landing = 26,
}
impl SoundCue {
    /// Original sound-bank mapping, retained at the resource boundary.
    pub const fn resource_id(self) -> usize {
        self as usize
    }
}
