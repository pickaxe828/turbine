# Turbine
Turbine, probably stands for: <br/>
***TUR***ing machine, \****B**urp* noise\*, ***I***t's ***N***ot an ***E***mulator

Basically a poorly coded configurable turing machine made with Rust, with hardcoded values and stuff. <br/>

## Start
1. Install prerequisites 📦
   * [Rust](https://www.rust-lang.org/tools/install)
2. Install dependencies and run 📥
   * Run `cargo run --release` in the terminal, packages will be install and ***LIFT-OFF!*** 🛫
3. Of course other than the default, you can also mess with the configs! 🔧🔥
   * Edit `config.turbine.toml` (Feel free to open an issue if you think anything should also be configurable)

## Config
* `[input]`: 
    * Input. What do you expect.
* `[behaviour]`
    * `output_iterations` Currently unused. Maybe in the future you can tell it to shut up with this. <br/>
Or maybe just shove an ANC headphone on your ears instead. So much peaceful, so much silence...
    * `reader_width` Determine how long is the tape reader.
* `[rules]`
    * Basically everything works.

## Sources
[Wikipedia (Rule 110)](https://en.wikipedia.org/wiki/Rule_110) \< You can also find other alternative rules here