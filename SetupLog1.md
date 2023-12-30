```text
PATH: </home/lamteteeow/STM32F4DISCOVERY_Rust/embedded/SetupLog1.txt>

Trying to learn Embedded Programming from bottom up without (or at least trying to) using any promoted external toolchain.
This file is to track things I absorbed as a mindflow for organized-recall learning method.

Book: Digital Signal Processing using Arm Cortex-M based Microcontrollers: Theory and Practice
    - Link to this book (code in C language): <https://www.arm.com/resources/ebook/digital-signal-processing>

Setting up dependencies:
    - Crate stm32f407g-disc v0.4.1 only supports Crate cortex-m-rt till v0.6.15 (depends originally on v0.6.13)
    => `cargo add stm32f407g-disc` (<https://crates.io/crates/stm32f407g-disc>)
    => `cargo add cortex-m-rt@0.6.15` (<https://crates.io/crates/cortex-m-rt/0.6.15>)
    - Crate cortex-m-semihosting (<https://crates.io/crates/cortex-m-semihosting>)

    READ THIS WELL: <https://lib.rs/crates/cargo-embed> as well as <https://github.com/probe-rs/probe-rs#building>
    - Installing Crate cargo-embed:
    => `cargo install cargo-embed`
    + Failed due to package libudev (Linux device manager) was not installed beforehand in WSL
    => `sudo apt-get install libudev-dev`c
    => Rerun => Worked! (for some issues on Github you might want to try: `cargo install --locked cargo-embed`)
    + For Windows, use crate "vcpkg"
        # dynamic linking 64-bit
            > `vcpkg install libftdi1:x64-windows libusb:x64-windows`
            > `set VCPKGRS_DYNAMIC=1`
        # static linking 64-bit
            > `vcpkg install libftdi1:x64-windows-static-md libusb:x64-windows-static-md`

    - Install Vim on WSL to edit and write files 
    - Use vim to add custom made rules (not system-installed rules) for udev (on Linux)
    => `cd /etc/udev/rules.d/`
    => `touch 49-stinkv2-1.rules`
    => `vim 49-stinkv2-1.rules`
    =>  ```text
        SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374a", \
            MODE:="0666", \
            SYMLINK+="stlinkv2-1_%n"

        SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", \
            MODE:="0666", \
            SYMLINK+="stlinkv2-1_%n"
        ```
    => ["ESC" + ":wq" + "ENTER"] to write file and quit
    => Reboot

    - cargo-embed depends on the libusb and optionally on [libftdi] libraries, which need to be installed to build cargo-embed:
    => sudo apt install -y pkg-config libusb-1.0-0-dev libftdi1-dev libudev-dev
    - probe-rs is better integrated toolset for embedded debugging with no GDB layer, and direct interface to the debug probe, which then enables other software to use its debug functionality.
        + Similar projects: OpenOCD, PyOCD, Segger Toolset, ST Tooling, ... They all implement the GDB protocol and their own protocol on top of it to enable GDB to communicate with the debug probe. 
        + Only Segger provides a closed source DLL which you can use for talking to the JLink.

    - READ: debug ports: JTAG vs SWD

    - READ: ST-Link [on-chip debug tool interface] vs OpenOCD [host] of (GNU Debugger - GDB) [debugger tool] => For Cross/Remote-debugging
        + GDB tool => OpenOCD [host] => ST-Link [hardware probe] => { JTAG/SWD => Cortex-A7 or **Cortex-M4** } as target
        + GDB tool => { Ethernet => gdbserver => Cortex-A7 }
    
    - READ: APIs for abstraction layer: LL Device Drivers (MCAL), HAL, and ECUAL (electronic control unit)
    - READ: communication interfaces: UART, I2C, SPI, CAN, GPIO(2 for I2C, 4 for SPI)
    - READ: MCU vs MPU (MPU: data processed by single or small number of IC/microchips)
    - READ: bootloader => U-Boot: universal bootloader
    - READ: FreeRTOS vs. CMSIS-RTOS => In case of doing RTOS
        + It’s important to understand how STM32CubeIDE has bundled FreeRTOS. While FreeRTOS is an underlying software framework that allows for switching tasks, scheduling, etc., we won’t be making calls to FreeRTOS directly.
        + ARM has created the CMSIS-RTOS library, which allows us to make calls to an underlying RTOS, thus improving the portability of code among various ARM processors. This image describes how ARM’s CMSIS libraries interact with third party software
        + FreeRTOS is mostly written in C, but there are Rust bindings for writing applications.
        + The Common Microcontroller Software Interface Standard (CMSIS) is a set of tools, APIs, frameworks, and work flows 
          that simplifies microcontroller software development, providing a consistent and efficient interface with Cortex-M and entry-level Cortex-A processors. 
        => CMSIS is also a Vendor-independent hardware abstraction layer for the Cortex-M processor series FRAMEWORK

Setting up tools:
    - Download Board Support and Device Support Pack for STM32F407VG from <https://www.keil.arm.com/packs/stm32f4xx_dfp-keil/boards/>
    - Use target-gen as a helper tool for probe-rs to extract flash algorithms and target descriptions for chips from ARM CMSIS-Packs
        => ??? target-gen not used ???
        => Assumption: target-gen crate is now merged into probe.rs => Therefore : cargo remove target-gen
    - Unpack the downloaded .pack file above (in case of .elf file usage please check probe.rs/target-gen on github)
        => `cargo run --release -- pack ../Keil.STM32F4xx_DFP.2.17.1.pack`
        => RESULT:
            Compiling semver-parser v0.7.0
            Compiling cortex-m v0.7.7
            Compiling proc-macro2 v1.0.69
            Compiling nb v1.1.0
            Compiling unicode-ident v1.0.12
            Compiling semver v1.0.20
            Compiling void v1.0.2
            Compiling vcell v0.1.3
            Compiling cortex-m-rt v0.6.15
            Compiling typenum v1.17.0
            Compiling version_check v0.9.4
            Compiling autocfg v1.1.0
            Compiling nb v0.1.3
            Compiling syn v1.0.109
            Compiling volatile-register v0.2.2
            Compiling embedded-hal v0.2.7
            Compiling bitfield v0.13.2
            Compiling libc v0.2.150
            Compiling semver v0.9.0
            Compiling r0 v0.2.2
            Compiling generic-array v0.14.7
            Compiling stm32f4 v0.13.0
            Compiling cfg-if v1.0.0
            Compiling num-traits v0.2.17
            Compiling stable_deref_trait v1.2.0
            Compiling embedded-dma v0.1.2
            Compiling cortex-m-semihosting v0.5.0
            Compiling rustc_version v0.2.3
            Compiling bare-metal v1.0.0
            Compiling stm32f407g-disc v0.4.1
            Compiling panic-halt v0.2.0
            Compiling bare-metal v0.2.5
            Compiling rustc_version v0.4.0
            Compiling cast v0.2.7
            Compiling quote v1.0.33
            Compiling panic-itm v0.4.2
            Compiling chrono v0.4.31
            Compiling micromath v1.1.1
            Compiling getrandom v0.2.11
            Compiling rand_core v0.6.4
            Compiling accelerometer v0.11.0
            Compiling lis302dl v0.1.0
            Compiling rtcc v0.2.1
            Compiling cortex-m-rt-macros v0.6.15
            Compiling stm32f4xx-hal v0.9.0
            Compiling embedded v0.1.0 (/home/lamteteeow/STM32F4DISCOVERY_Rust/embedded)
             Finished release [optimized] target(s) in 17.72s
              Running `target/release/embedded pack ../Keil.STM32F4xx_DFP.2.17.1.pack`

    - Installing probe-rs:
        => `cargo install probe-rs --features cli`
        + error: binary `cargo-embed` already exists in destination as part of `cargo-embed v0.18.0` 
        => `cargo uninstall cargo-embed`
        => `cargo install probe-rs --features cli(,ftdi)` # for ftdi support
        + failed because command changed to:
        => `cargo install probe-rs --features="cli"`
        => Work! Now there are 3 new executables in CLI: cargo-embed, cargo-flash and probe-rs

    - Download udev rules from <https://probe.rs/docs/getting-started/probe-setup/> and put to /etc/udev/rules.d
        => `sudo mv 69-probe-rs.rules /etc/udev/rules.d`
        => `sudo udevadm control --reload` (to make sure rules are used)
        => `sudo udevadm trigger` (to apply rules to added devices)
    - CMSIS-DAP [debug access] is a standard for debug probes which is managed by ARM. All probes implementing this standard are supported by probe-rs.
        => No further action requires after udev rules are applied
    - ST-Link [debug probe from ST Microelectronics] as board default (not DAPLink or JLink) for Ubuntu
        => No further action requires after udev rules are applied
    - In case of J-Link [debug probe from Segger]
        => No further action requires after udev rules are applied
        
    - Add .cargo/config.toml to project folder, then add target to config.toml (this file could be created within project as .cargo/config.toml). This aims to use "cargo run" 
        => Template:  [target.<architecture-triple>]
                      runner = "probe-rs run --chip <chip-name>"
    - Check all supported %PROCESSOR_ARCHITECTURE%:
        => `rustup target list`
    - Check online for ARM Cortex-M4
        => thumbv7em-none-eabi (replace <architecture-triple) 
            + without FPU support
        => thumbv7em-none-eabihf (instead of above)
    - Install the rust-std component for that target above:
        => `rustup target add thumbv7em-none-eabihf`
        => `rustup target list` (to check if it is installed: default should include your host machine as well)
        => [x86_64-unknown-linux-gnu] and [thumbv7em-none-eabihf] INSTALLED
        NOTE: This has supported target descriptions. 
              In case of not supported, write your own flash algorithm. Follow: https://github.com/probe-rs/flash-algorithm-template
    - Check online for standard chip name, then add these to config.toml file :
        ```toml
        [target.thumbv7em-none-eabihf]
        runner = "probe-rs run --chip <chip-name>" (chip-name is "STM32F407VGTx")
        ```


Setup connection of USB devices to WSL: https://learn.microsoft.com/en-us/windows/wsl/connect-usb or https://devblogs.microsoft.com/commandline/connecting-usb-devices-to-wsl/
    - Use Windows package manager (winget) in cmd to run: 
        => `winget install --interactive --exact dorssel.usbipd-win`
        => Restart PC
    - On Ubuntu terminal run to install the user space tools and a database of USB hardware identifiers: 
        =>  `sudo apt install linux-tools-generic hwdata`
        =>  `sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/*-generic/usbip 20`
    - Keep WSL command line open to keep the WSL 2 lightweight VM active
    - List all of the USB devices connected to Windows by opening PowerShell in ADMINISTRATOR mode and run in cmd:
        => `usbipd wsl list` (for our microprocessor it should show "STM32 STLink")
        => Remember the BUSID of selected USB device (here was "1-2")
    - Attach that USB device to the default Linux WSL distribution (here Ubuntu):
        => `usbipd wsl attach --busid <busid>` (here is "1-2")
    - Go to WSL terminal and list all attached USB devices:
        => `lsusb`
    - For detaching, either physically do it or run this in cmd:
        => `usbipd wsl detach --busid <busid>`
    

Optional:
    - Install Cargo "clone" subcommand: 
        => `cargo install cargo-clone`
    - Install Cargo "cargo-add, cargo-rm, cargo-set-version, cargo-upgrade" from cargo-edit subcommand:
        => `cargo install cargo-edit`
    - GDB: `sudo apt-get install gdb-arm-none-eabi` (on Ubuntu)
    - OpenOCD: `sudo apt-get install OpenOCD` (on Ubuntu)

HUGE REALIZATION: STM32F407VG does not support Windows 11 ... So do its required drivers.
Things were done that might need to be revised later:
    - Drivers STSW-LINK009 was downloaded from <https://www.st.com/en/development-tools/stsw-link009.html>
    - Ran and installed "stlink_winusb_install.bat" from that .zip folder
    - Installed all .inf files from that .zip folder
    - Now try out a correct Mini-USB cable to power up => UP YOU GO!!!

NOW follow this book to learn embed in Rust <https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html>
=> Move to a new project name rust-embedded for this book
=> This project here is for self implementing after learning the book
```

# Next doc: </home/lamteteeow/STM32F4DISCOVERY_Rust/test-size/SetupLog2.txt>
