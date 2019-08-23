# Epicardium Rust Bindings

**Experimental and unfinished, _not all Epicardium API functions have been mapped and there is nothing final about the API design_**

This is Rust crate intends to make the [Epicardium API](https://firmware.card10.badge.events.ccc.de/epicardium/overview.html) for your CCC Camp 2019 wristband [card10](https://git.card10.badge.events.ccc.de/card10) available to Rust.
Its **WORK IN PROGRESS** API should be similar to the [Python-API](https://firmware.card10.badge.events.ccc.de/pycardium/overview.html) and the goal is to make it easier to create your own [l0adable](https://firmware.card10.badge.events.ccc.de/overview.html#l0dables) by using a quickstart/template project cross-compiling it.

## Inspiration

This code only works because of the great groundwork of [https://git.card10.badge.events.ccc.de/astro/rust-card10](https://git.card10.badge.events.ccc.de/astro/rust-card10). 

## Modules progress

Epicardium has several modules which need to be wrapped. The low-level C bindings in the `epicardium-sys` crate are automatically generated with [bindgen](https://github.com/rust-lang/rust-bindgen), but these should also be wrapped in a nicer, more Rust-like API. **A checkbox here means the code is implemented, not that it has ever been tested if its actually working on the device**.

- [X] display
- [ ] leds
- [ ] light_sensor
- [ ] utime
- [ ] vibra
- ...

## API documentation

Generate the documentation from you checkout with 

```bash
cargo doc --no-deps
```

and open the resulting file `target/doc/rustcardium/index.html`.

## Example

There is an example in the `example/` folder which shows a plot of the accelerometer values.