# Todo
* [x] Debug a "rare" occurrence of a turret shooting many projectiles at the same time.
* [x] Debug monster spawns.
* [x] Refactor Assets management.
* [x] Extract AI into its own module.
* [x] Make Turrets sensor follow the target when in TargetAcquired mode.
* [ ] Move the Target related logic from Gun to AI.
* [ ] Implement a soft target release mechanism; after a short period of no longer detecting the target, the turret should be put in stand by (instead of checking coordinates manually).
* [ ] Implement mobile Turrets!
* [ ] Implement a "smart grid" so that a turret shares information on targets once detected.
* [x] Implement laser turrets.
* [ ] Debug laser collision detection.
* [ ] Improve target detection (it's only using the center of the target right now).
  * Replace check using the center pixel by the closest edge pixel.
* [x] Implement a custom fire mode, normal or burst
* Add sounds.
  * [ ] One when turret changes direction (reaches left or right gun boundary)
  * [ ] One for a rotating sensor (it should be subtle and "humming")
  * [ ] One for firing a projectile
  * [ ] One when a projectile reaches its target
  * [ ] One when a monster dies
* Add "real" game mechanics:
  * [ ] Implement cooldowns on turret deployments
  * [ ] Reset/game over when a monster reaches the bottom.
  * [ ] Score over time, plus a bonus for a killed monster.
  * [ ] Have reliable monster spawns.
