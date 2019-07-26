# Epicardium Rust Bindings

**Experimental and unfinished, not even tested on the actual hardware yet!**

This is Rust crate which intends to make the [Epicardium API](https://firmware.card10.badge.events.ccc.de/epicardium/overview.html) for your CCC Camp 2019 wristband [card10](https://git.card10.badge.events.ccc.de/card10) available to Rust.

## How to use

1. Use [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) to create project which can produce Cortex-M binaries
2. Configure `thumbv7em-none-eabihf` as a target
3. Use the following configuration in the `memory.x` file
    - FLASH : ORIGIN = **0x10080000**, LENGTH = **512K**
    - RAM : ORIGIN = **0x20040000**, LENGTH = **256K**
4. Add this crate as dependency (TODO: where is it published?)
5. TODO: create a combined firmware binary from your project and the Epicardium firmware
