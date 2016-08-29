# La Guerra Impresionante de Condenación Final
A small game I'm writing to learn Rust

Design your own weapons, shoot and kill your family, friends and coworkers. Identify as a square of bright colors. Press buttons. Have fun.

##Current status

![2016-06-21](https://raw.githubusercontent.com/joelwkall/lgidcf/master/screenshots/2016-06-21.png "2016-05-26")

##TODO

* [ ] More shapes of weapons (~~circle~~, ~~rectangle~~, triangle)
* [X] Rotation of shape (will make hit detection with obstacles harder)
* [X] More events for weapons (timed)
* [X] Multiple weapons
* [X] Life bars
* [ ] Jetpack fuel bars
* [ ] Scorekeeping
* [X] Fullscreen
* [X] Deployment on computers without Rust installed (works with MSVC++ redist on windows)
* [ ] Sound effects (using synth)
* [ ] Weapon builder
* [ ] Performance tuning (Very Sleepy? use glium or gfx directly instead of piston?)
* [ ] Menus and stuff
* [ ] Background image (parallax scrolling?)
* [ ] Obstacles (destructible?)
* [X] Player names
* [ ] Modify property on event (using set, add or multiply)
* [ ] Gradient colors (and alpha)
* [X] Acceleration property
* [X] Speed additive or multiplicative (for different types of projectiles)
* [ ] Recoil and impact force
* [ ] Tests
* [X] Gravity property
* [ ] Music!

##Design decisions

- Jetpack as a device instead? (requires recoil)
- Large map with zoom in on players?
- Size of player relative to health?
- Images as players/projectiles/obstacles or keep it simple?