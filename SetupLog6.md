## probe-rs for VSCode

Start setting up and using probe-rs with RTT, defmt for debugging without going through GDB

- Installing Debugger for probe-rs extension on VSCode

- Installing probe-rs: `cargo install probe-rs --features="cli"`

- Installing probe-run: `cargo install probe-run`

- Check all supported chips: `probe-run --list-chips` (ours is STM32F407VGTx)

- Update firmware on ST-Link V2 using: <https://www.st.com/en/development-tools/stsw-link007.html>

- Update new udev rules for Linux: `sudo dpkg -i st-stlink-udev-rules-1.0.3-2-linux-all.deb`

- Configure the <.cargo/config.toml> file:

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
# runner = "gdb-multiarch -q -x openocd.gdb" # run gdb in root of project

[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-run --chip STM32F407VGTx" # or "probe-rs run --chip STM32F407VGTx"
rustflags = [
    # LLD (shipped with the Rust toolchain) is used as the default linker
    "-C",
    "link-arg=-Tlink.x",
    # This is needed if your flash or ram addresses are not aligned to 0x10000 in memory.x
    # See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    # "-C",
    # "link-arg=--nmagic",
]
```

- Download corresponding SVD file: <https://www.st.com/en/microcontrollers-microprocessors/stm32f407vg.html#cad-resources>

- Start DAP Protocol server: `probe-rs dap-server --port 50000`
