# pico-flasher

Simple rust tool to flash ELF files, possibly compiled from a rust project, onto an RP2040 or similar chip.

To flash a file to a pico board connected via USB in bootsel mode run:

```shell
pico_flasher PATH_TO_ELF_FILE
```

## TODO

* Expose this as a library as well as a binary
* `cargo run` integration

## License

MIT
