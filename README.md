# Goals
- make a multiplayer 2D game using bevy
- make the game fun
- document the process 
## Original Goals (abandoned)
- make a multiplayer game engine from scratch 
- write an interesting game to run on top of 
- make easy to read comments that set a precedent to making a game from scratch in rust
- create a multiplayer game client/server paradigm using Apache Kafka
- have fun
### What happened
The original goals were abandoned due to the regrettable lack of SDL support, maintained crates for vulkan/opengl. I've tried to use several crates such as Vulkano, glium, gl, and so on. The common breaking point in an investigation into any of these is documentation and examples. Vulkano has an incredibly dense book but is virtually incompatible with winit or the SDL2 crate for rust. Glium's documentation has been in a state of pallid disrepair for the last 7 years, while direct OpenGL function calls work for small examples but would only lead to tens if not hundreds of hours wasted. Then there was Ash, seemingly functional but so low level compared to C++ opengl that all intentions to use a direct bindings crate were abandoned. The only well supported and feasible option remaining was wgpu-rs. At that point, however, it was arguably wiser not to reinvent the wheel and build on top of bevy.   

# TODO
## Simply learning graphics and the tools available
- [ ] get a bevy window working and displaying a couple textures
- [ ] program the Apache Kafka interface for multiplayer
- [ ] TODO!
