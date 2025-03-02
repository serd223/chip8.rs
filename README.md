# chip8.rs

`CHIP-8 is an interpreted minimalist programming language that was designed by Joseph Weisbecker in the 1970s for use on the RCA COSMAC VIP computer.`
    ([source](https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908))


`chip8.rs` is a library that lets you easily interpret chip-8 programs with your own custom runtime. The library is `no_std`, so you can use it wherever you want!

You only need to provide the program you want to interpret and handle the platform-specific stuff (displaying the framebuffer, handling user input, timing and playing sounds). `chip8.rs` will take care of interpreting the logic!

# Examples
The `simple` example inside the `examples` folder is an example runtime which contains all necessary logic for a chip8 runtime except for sound (TODO).
## Build Instructions
### Prerequisites
* Linux
  - Dependencies required by the minifb crate (used for rendering):
  ```console
    $ sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
  ```
### Instructions
```console
  $ cargo run --example simple --release ./your_chip8_program.ch8
```

# Using chip8.rs in your projects
You can use the `cargo add` command:
```console
  cargo add --git https://github.com/serd223/chip8.rs
```
Or alternatively, you can add the following to your `Cargo.toml`:
```toml
  chip8 = { git = "https://github.com/serd223/chip8.rs"}
```

# Resources
These are the resources I used to learn about chip-8 itself and implement `chip8.rs`:
 * https://chip-8.github.io/links/
 * https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908
 * http://johnearnest.github.io/Octo/docs/chip8ref.pdf
 * https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
 * https://github.com/kripod/chip8-roms
