use crate::common::recurrence::Recurrence;

// Holds a collection of actions and just keeps rerunning them.
pub struct LoopAction {
    recurrence: Recurrence,
}

// TODO: See “repeat” and “parallel” examples from bevy-sequential-actions.
