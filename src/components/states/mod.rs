// TODO: How to convert all these into reusable states?

// Goals:
// * Generate action lists in on_enter(), avoid update loop as much as possible!
// * Only run pathing logic once per state to generate movement actions.
// * Use events to trigger Stunned, ChasingPlayer.
// * Exploit how states are just components, and are treated as such by systems?

// Stunned
// Transitions:
// In:
// AnyState
// hit_by_stun_bullet
// Stunned
// Out:
// Stunned
// done(None)
// Patrolling, GuardingLocation

// on_enter:
// ParallelAction:
// DialogBubbleAction with image of “?”.
// AnimationAction of stunned animation.
// DoneAction.

// ChasingPlayer
// player_location: Vec3

// Transitions:
// In:
// (SearchingForPlayer, InvestigatingSound, Guarding)
// saw_player
// ChasingPlayer

// on_enter:
// ParallelAction:
// DialogBubbleAction with image of red on yellow “!”.
// AnimationAction of reaction animation.
// DialogBubbleAction with image of yellow on orange “!”.
// <loop over path to player>
// MoveToAction toward path point.
// DoneAction.

// SearchingForPlayer
// Transitions:
// In:
// ChasingPlayer
// done(None)
// SearchingForPlayer
// Out:
// SearchingForPlayer
// done(None)
// Patrolling, GuardingLocation

// on_enter:
// RotateToFaceDirectionAction  with randomly generated direction.
// WaitAction
// RotateToFaceDirectionAction  with randomly generated direction.
// ParallelAction:
// DialogBubbleAction with angry black squiggles on a yellow bubble.
// AnimationAction starts playing confused/looking around animation.
// DoneAction.

// Must be separate from ChasingPlayer so it can resume if Player is spotted again! Can’t handle return to start logic since we need Guards to be able to reset after investigating a sound.

// InvestigatingSound
// sound_location: Vec3

// Transitions:
// In:
// Guarding
// heard_a_sound
// InvestigatingSound
// Out:
// InvestigatingSound
// done(None)
// Patrolling, GuardingLocation

// on_enter:
// RotateToFaceDirectionAction  with direction from entity to location provided by trigger.
// AnimationAction starts playing confused/looking around animation.
// DoneAction.

// Guarding::Patrolling
// on_enter:
// <loop over path to starting location>
// MoveToAction to each point.
// LoopAction:
// <loop over patrol route waypoints>
// <loop over points in generated path from guard to waypoint>
// MoveToAction to each point.
// RotateToFaceDirectionAction  toward waypoint direction.
// AnimationAction of Idle animation for a few seconds.
// <exit loop when we’ve returned to start of patrol route>

// Guarding::Location
// <loop over path to starting location>
// MoveToAction to each point.
// RotateToFaceDirectionAction using starting location direction.
// LoopAction:
// AnimationAction of Idle animation forever.

// Raising
// Transitions:
// In:
// AnyState
// on_event::<TriggerBarrier>
// Raising
// Out:
// Raising
// done(None)
// Idle

// on_enter:
// Raise barrier.
// DoneAction.

// Lowering
// Transitions:
// In:
// AnyState
// on_event::<TriggerBarrier>
// Lowering
// Out:
// Lowering
// done(None)
// Idle

// on_enter:
// Lower barrier.
// DoneAction.

// Idle
// Do nothing.

// PanningLeft
// Transitions:
// In:
// PanningRight
// done(None)
// PanningLeft

// on_enter:
// Pan left.
// DoneAction.

// PanningRight
// Transitions:
// In:
// PanningLeft
// done(None)
// PanningRight

// on_enter:
// Pan right.
// DoneAction.
