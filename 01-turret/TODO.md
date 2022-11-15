# Todo
* Add sounds.
  * [ ] One when turret changes direction (reaches left or right gun boundary)
  * [ ] One for a rotating sensor (it should be subtle and "humming")
  * [x] One for firing a projectile
  * [ ] One when a projectile reaches its target
  * [ ] One when a monster dies
* [ ] Add "real" game mechanics:
  * Reset/game over when a monster reaches the bottom.
  * Score over time, plus a bonus for a killed monster.
  * Have reliable monster spawns.
* [x] Debug a "rare" occurrence of a turret shooting many projectiles at the same time.
* [x] Debug monster spawns.
* [x] Refactor Assets management.
* [ ] Implement a soft target release mechanism; after a short period of no longer detecting the target, the turret should be put in stand by (instead of checking coordinates manually).
