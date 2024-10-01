# Bevy Stealth Game Example

My attempt to mimic the game mechanics of "SONAR" in the Bevy engine. It's gonna be mostly pseudo-code for a while, so be patient.

## ToDo

- [ ] Add sound loading and management via bevy_kira_audio.
- [ ] Add background music via bevy_kira_audio and CC songs.
- [ ] Use bevy-tnua character controller for Player movement & collisions.
- [ ] Create level deserialization and spawning.
- [ ] Make custom level format and load from it.
- [ ] Set up embedded scripts as enum arrays in level file format.
- [ ] Add level navmesh and collision mesh generation.
- [ ] Multiple level loading and transitions.
- [ ] Implement Guard patrol behaviors via level scripts.
- [ ] Implement triggers via level scripts.
- [ ] Use smooth-bevy-cameras to make camera follow the player.
- [ ] Pickup implementation.
- [ ] Weapons implementation including bounce UI.
- [ ] Emote effects via bevy_hanabi.
- [ ] Enemy vision cone and detection.
- [ ] Enemy hearing circle and detection.
- [ ] Enemy cameras.
- [ ] Enemy alarm.
- [ ] Add enemy alert level and doors that respond to it.
- [ ] Add wall shatter effects.
- [ ] Footstep effects via bevy_hanabi.
- [ ] Sound wave physics and logic.
- [ ] Use bevy_fluent to implement localization support and migrate all user-facing strings.
- [ ] Add basic menus via bevy_egui.
- [ ] Add UI animations via bevy_tweening.
- [x] Load entities from YAML files rather than hard-coding them.
- [x] Spawning entities from config files.
- [x] Animated mesh entity.
- [x] Use Bevy animation demo fox model to test complex animating entities.
- [x] Automatic animation based on movement.
- [x] Smooth blended transitions between animations.
- [x] Use leafwing-input-manager to add Player input movement.
- [x] Replace Spew crate with Bevy observers.
- [x] Use vleue_navigator for nav-mesh pathing.
- [x] Figure out why first MoveToAction in sequences gets ignored!
- [x] Finish porting animation system to Bevy AnimationGraph API.
