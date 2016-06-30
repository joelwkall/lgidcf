# La Guerra Impresionante de Condenación Final
A small game I'm writing to learn Rust

Design your own weapons, shoot and kill your family, friends and coworkers. Identify as a square of bright colors. Press buttons. Have fun.

##Current status

![2016-06-21](https://raw.githubusercontent.com/joelwkall/lgidcf/master/screenshots/2016-06-21.png "2016-05-26")

##TODO

* [ ] More shapes of weapons (circle, rectangle, triangle)
* [ ] Rotation of shape
* [X] More events for weapons (timed)
* [ ] Multiple weapons
* [X] Life bars
* [ ] Jetpack fuel bars
* [ ] Scorekeeping
* [ ] Fullscreen
* [X] Deployment on computers without Rust installed (works with MSVC++ redist on windows)
* [ ] Sound effects (using synth)
* [ ] Weapon builder
* [ ] Performance tuning (use glium or gfx-rs instead of piston?)
* [ ] Menus and stuff
* [ ] Background image
* [ ] Obstacles
* [ ] Player names
* [ ] Sound effects on events (wavetype, length, pitch, volume)
* [ ] Modify properties on events (speed, acceleration, shape, color)
* [ ] Gradient colors (and alpha)
* [X] Acceleration property
* [ ] Speed additive or multiplicative (for different types of projectiles)

##Design decisions

- Jetpack as a device instead?
- Large map with zoom in on players?