# Bevy Stealth Sound Game

My attempt to mimic the game mechanics of "SONAR" in the Bevy engine. It's gonna be mostly pseudo-code for a while, so be patient.

## ToDo

- [ ] Switch to Bevy component lifecycle hooks.
- [ ] Finish porting animation system to Bevy AnimationGraph API.
- [ ] Use vleue_navigator for nav-mesh pathing.
- [ ] Use bevy_rapier character controller integration.
- [ ] Waypoint patrolling.
- [ ] Use smooth-bevy-cameras to make camera follow the player.
- [ ] Make custom level format and load from it.
- [ ] Multiple level loading.
- [ ] Pickup implementation.
- [ ] Weapons implementation.
- [ ] Emote effects.
- [ ] Enemy vision.
- [ ] Sound wave physics and logic.
- [ ] Use bevy_fluent to implement localization support and migrate all user-facing strings.
- [ ] Basic UI.
- [ ] Menu animations.
- [x] Load entities from YAML files rather than hard-coding them.
- [x] Spawning entities from config files.
- [x] Animated mesh entity.
- [x] Use Bevy animation demo fox model to test complex animating entities.
- [x] Automatic animation based on movement.
- [x] Smooth blended transitions between animations.
- [x] Use leafwing-input-manager to add Player input movement.
- [x] Replace Spew crate with Bevy observers.
