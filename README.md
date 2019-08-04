# Epicardium Rust Bindings

**Experimental and unfinished, _not even tested on the actual hardware yet!_**

This is Rust crate intends to make the [Epicardium API](https://firmware.card10.badge.events.ccc.de/epicardium/overview.html) for your CCC Camp 2019 wristband [card10](https://git.card10.badge.events.ccc.de/card10) available to Rust.
Its **WORK IN PROGRESS** API should be similar to the [Python-API](https://firmware.card10.badge.events.ccc.de/pycardium/overview.html) and the goal is to make it easier to create your own [l0adable](https://firmware.card10.badge.events.ccc.de/overview.html#l0dables) by using a quickstart/template project based on the [cortex-m](https://github.com/rust-embedded/cortex-m) crates and cross-compiling it.

## Modules progress

Epicardium has several modules which need to be wrapped. The low-level C bindings in the `epicardium-sys` crate are automatically generated with [bindgen](https://github.com/rust-lang/rust-bindgen), but these should also be wrapped in a nicer, more Rust-like API. **A checkbox here means the code is implemented, not that it has ever been tested if its actually working on the device**.

- [X] display
- [ ] leds
- [ ] light_sensor
- [ ] utime
- [ ] vibra

## API documentation

Generate the documentation from you checkout with 

```bash
cargo doc --no-deps
```

and open the resulting file `target/doc/rustcardium/index.html`.


## How it might be used (after I can test it)

1. Use [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart) to create project which can produce Cortex-M binaries
2. Configure `thumbv7em-none-eabihf` as a target
3. Use the following configuration in the `memory.x` file
    - FLASH : ORIGIN = **0x10080000**, LENGTH = **512K**
    - RAM : ORIGIN = **0x20040000**, LENGTH = **256K**
4. Add this crate as dependency. You can use this git repository until it might be published on crates.io
```toml
[dependencies]
rustcardium = { git = "https://github.com/thomaskrause/rustcardium" }
```
5. TODO: Store this as a l0adable on the card10 and start it
**THIS HAS NOT BEEN TESTED YET**
