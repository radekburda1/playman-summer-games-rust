//! Mutually exclusive event data. A game owns exactly one discipline.
use crate::{
    simulation::Event, simulation::Phase, simulation::events::javelin::Javelin,
    simulation::events::long_jump::LongJump, simulation::events::race::Hurdles,
    simulation::events::vault::Vault,
};
#[derive(Clone, Debug)]
pub enum Discipline {
    Dash(RacePhase),
    LongJump(LongJump),
    Hurdles(Hurdles),
    PoleVault(Vault),
    Javelin(Javelin),
}
impl Discipline {
    pub fn new(event: Event) -> Self {
        match event {
            Event::Dash => Self::Dash(RacePhase::Run),
            Event::LongJump => Self::LongJump(LongJump::default()),
            Event::Hurdles => Self::Hurdles(Hurdles::default()),
            Event::PoleVault => Self::PoleVault(Vault::default()),
            Event::Javelin => Self::Javelin(Javelin::default()),
        }
    }
    pub fn event(&self) -> Event {
        match self {
            Self::Dash(_) => Event::Dash,
            Self::LongJump(_) => Event::LongJump,
            Self::Hurdles(_) => Event::Hurdles,
            Self::PoleVault(_) => Event::PoleVault,
            Self::Javelin(_) => Event::Javelin,
        }
    }
}

impl Discipline {
    /// Checked access for callers that have not already matched the discipline.
    pub fn as_vault(&self) -> Option<&Vault> {
        match self {
            Self::PoleVault(state) => Some(state),
            _ => None,
        }
    }
    // Internal callers dispatch on event identity before requesting event data.
    pub(crate) fn vault(&self) -> &Vault {
        match self {
            Self::PoleVault(state) => state,
            _ => unreachable!("vault data requested for another discipline"),
        }
    }
    pub(in crate::simulation) fn vault_mut(&mut self) -> &mut Vault {
        match self {
            Self::PoleVault(state) => state,
            _ => unreachable!("vault data requested for another discipline"),
        }
    }
}
impl Discipline {
    /// Checked access for callers that have not already matched the discipline.
    pub fn as_javelin(&self) -> Option<&Javelin> {
        match self {
            Self::Javelin(state) => Some(state),
            _ => None,
        }
    }
    // Internal callers dispatch on event identity before requesting event data.
    pub(crate) fn javelin(&self) -> &Javelin {
        match self {
            Self::Javelin(state) => state,
            _ => unreachable!("javelin data requested for another discipline"),
        }
    }
    pub(in crate::simulation) fn javelin_mut(&mut self) -> &mut Javelin {
        match self {
            Self::Javelin(state) => state,
            _ => unreachable!("javelin data requested for another discipline"),
        }
    }
}
impl Discipline {
    /// Checked access for callers that have not already matched the discipline.
    pub fn as_long_jump(&self) -> Option<&LongJump> {
        match self {
            Self::LongJump(state) => Some(state),
            _ => None,
        }
    }
    // Internal callers dispatch on event identity before requesting event data.
    pub(crate) fn long_jump(&self) -> &LongJump {
        match self {
            Self::LongJump(state) => state,
            _ => unreachable!("long_jump data requested for another discipline"),
        }
    }
    pub(in crate::simulation) fn long_jump_mut(&mut self) -> &mut LongJump {
        match self {
            Self::LongJump(state) => state,
            _ => unreachable!("long_jump data requested for another discipline"),
        }
    }
}
impl Discipline {
    /// Checked access for callers that have not already matched the discipline.
    pub fn as_hurdles(&self) -> Option<&Hurdles> {
        match self {
            Self::Hurdles(state) => Some(state),
            _ => None,
        }
    }
    // Internal callers dispatch on event identity before requesting event data.
    pub(crate) fn hurdles(&self) -> &Hurdles {
        match self {
            Self::Hurdles(state) => state,
            _ => unreachable!("hurdles data requested for another discipline"),
        }
    }
    pub(in crate::simulation) fn hurdles_mut(&mut self) -> &mut Hurdles {
        match self {
            Self::Hurdles(state) => state,
            _ => unreachable!("hurdles data requested for another discipline"),
        }
    }
}

/// A phase from another discipline cannot be installed in this event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidPhase;
#[derive(Clone, Copy, Debug, Default)]
pub enum RacePhase {
    #[default]
    Run,
    Done,
}
impl From<RacePhase> for Phase {
    fn from(value: RacePhase) -> Self {
        match value {
            RacePhase::Run => Phase::Run,
            RacePhase::Done => Phase::Done,
        }
    }
}
impl TryFrom<Phase> for RacePhase {
    type Error = InvalidPhase;
    fn try_from(value: Phase) -> Result<Self, Self::Error> {
        match value {
            Phase::Run => Ok(Self::Run),
            Phase::Done => Ok(Self::Done),
            _ => Err(InvalidPhase),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub enum LongJumpPhase {
    #[default]
    Prepare,
    Run,
    Approach,
    Air,
    Stretch,
    BadStretch,
    FoulRun,
    LandGood,
    LandBad,
    Done,
}
impl From<LongJumpPhase> for Phase {
    fn from(value: LongJumpPhase) -> Self {
        match value {
            LongJumpPhase::Prepare => Phase::Prepare,
            LongJumpPhase::Run => Phase::Run,
            LongJumpPhase::Approach => Phase::Approach,
            LongJumpPhase::Air => Phase::Air,
            LongJumpPhase::Stretch => Phase::Stretch,
            LongJumpPhase::BadStretch => Phase::BadStretch,
            LongJumpPhase::FoulRun => Phase::FoulRun,
            LongJumpPhase::LandGood => Phase::LandGood,
            LongJumpPhase::LandBad => Phase::LandBad,
            LongJumpPhase::Done => Phase::Done,
        }
    }
}
impl TryFrom<Phase> for LongJumpPhase {
    type Error = InvalidPhase;
    fn try_from(value: Phase) -> Result<Self, Self::Error> {
        match value {
            Phase::Prepare => Ok(Self::Prepare),
            Phase::Run => Ok(Self::Run),
            Phase::Approach => Ok(Self::Approach),
            Phase::Air => Ok(Self::Air),
            Phase::Stretch => Ok(Self::Stretch),
            Phase::BadStretch => Ok(Self::BadStretch),
            Phase::FoulRun => Ok(Self::FoulRun),
            Phase::LandGood => Ok(Self::LandGood),
            Phase::LandBad => Ok(Self::LandBad),
            Phase::Done => Ok(Self::Done),
            _ => Err(InvalidPhase),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub enum VaultPhase {
    #[default]
    Prepare,
    Run,
    VaultPlant,
    VaultBend,
    VaultStraighten,
    VaultRelease,
    VaultFall,
    Done,
}
impl From<VaultPhase> for Phase {
    fn from(value: VaultPhase) -> Self {
        match value {
            VaultPhase::Prepare => Phase::Prepare,
            VaultPhase::Run => Phase::Run,
            VaultPhase::VaultPlant => Phase::VaultPlant,
            VaultPhase::VaultBend => Phase::VaultBend,
            VaultPhase::VaultStraighten => Phase::VaultStraighten,
            VaultPhase::VaultRelease => Phase::VaultRelease,
            VaultPhase::VaultFall => Phase::VaultFall,
            VaultPhase::Done => Phase::Done,
        }
    }
}
impl TryFrom<Phase> for VaultPhase {
    type Error = InvalidPhase;
    fn try_from(value: Phase) -> Result<Self, Self::Error> {
        match value {
            Phase::Prepare => Ok(Self::Prepare),
            Phase::Run => Ok(Self::Run),
            Phase::VaultPlant => Ok(Self::VaultPlant),
            Phase::VaultBend => Ok(Self::VaultBend),
            Phase::VaultStraighten => Ok(Self::VaultStraighten),
            Phase::VaultRelease => Ok(Self::VaultRelease),
            Phase::VaultFall => Ok(Self::VaultFall),
            Phase::Done => Ok(Self::Done),
            _ => Err(InvalidPhase),
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub enum JavelinPhase {
    #[default]
    Prepare,
    Run,
    Approach,
    Charge,
    Throw,
    Flight,
    FoulRun,
    Done,
}
impl From<JavelinPhase> for Phase {
    fn from(value: JavelinPhase) -> Self {
        match value {
            JavelinPhase::Prepare => Phase::Prepare,
            JavelinPhase::Run => Phase::Run,
            JavelinPhase::Approach => Phase::Approach,
            JavelinPhase::Charge => Phase::Charge,
            JavelinPhase::Throw => Phase::Throw,
            JavelinPhase::Flight => Phase::Flight,
            JavelinPhase::FoulRun => Phase::FoulRun,
            JavelinPhase::Done => Phase::Done,
        }
    }
}
impl TryFrom<Phase> for JavelinPhase {
    type Error = InvalidPhase;
    fn try_from(value: Phase) -> Result<Self, Self::Error> {
        match value {
            Phase::Prepare => Ok(Self::Prepare),
            Phase::Run => Ok(Self::Run),
            Phase::Approach => Ok(Self::Approach),
            Phase::Charge => Ok(Self::Charge),
            Phase::Throw => Ok(Self::Throw),
            Phase::Flight => Ok(Self::Flight),
            Phase::FoulRun => Ok(Self::FoulRun),
            Phase::Done => Ok(Self::Done),
            _ => Err(InvalidPhase),
        }
    }
}
impl Discipline {
    pub fn phase(&self) -> Phase {
        match self {
            Self::Dash(phase) => (*phase).into(),
            Self::LongJump(state) => state.phase.into(),
            Self::Hurdles(state) => state.phase.into(),
            Self::PoleVault(state) => state.phase.into(),
            Self::Javelin(state) => state.phase.into(),
        }
    }
    pub(in crate::simulation) fn set_phase(&mut self, phase: Phase) -> Result<(), InvalidPhase> {
        match self {
            Self::Dash(current) => *current = phase.try_into()?,
            Self::LongJump(state) => state.phase = phase.try_into()?,
            Self::Hurdles(state) => state.phase = phase.try_into()?,
            Self::PoleVault(state) => state.phase = phase.try_into()?,
            Self::Javelin(state) => state.phase = phase.try_into()?,
        }
        Ok(())
    }
}
