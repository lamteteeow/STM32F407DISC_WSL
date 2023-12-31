#### 1. NOW follow this book to learn embed in Rust <https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html>

#### 2. Installing tools

```text
    - Check rustc version: 
        => rustc -V
    - A library and tool (itmdump) to parse and dump ARM ITM packets
        => cargo install itm 
    - Install cargo-binutils: 
        => rustup component add llvm-tools-preview (LLVM is a library that is used to construct, optimize and produce intermediate and/or binary machine code)
        => cargo install cargo-binutils (Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain)
            New executables `cargo-cov`, `cargo-nm`, `cargo-objcopy`, `cargo-objdump`, 
                            `cargo-profdata`, `cargo-readobj`, `cargo-size`, `cargo-strip`, 
                            `rust-ar`, `rust-cov`, `rust-ld`, `rust-lld`, `rust-nm`, `rust-objcopy`, 
                            `rust-objdump`, `rust-profdata`, `rust-readobj`, `rust-size`, `rust-strip`
    - New project
        => cargo new test-size
            (base) lamteteeow@Lami:~/STM32F4DISCOVERY_Rust/test-size$ cargo size -- --version
            Finished dev [unoptimized + debuginfo] target(s) in 0.00s
            /home/lamteteeow/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-size
            LLVM (http://llvm.org/):
            LLVM version 17.0.2-rust-1.73.0-stable
            Optimized build.
```

#### 3. Installing packages

```text
    - "gdb-multiarch" is the GDB command you'll use to debug your ARM Cortex-M programs, specifically for Ubuntu >18.04 distro
        => sudo apt-get install \
           gdb-multiarch \
           minicom \
           openocd
    - Optional: 
        => sudo apt-get install \
           bluez \
           rfkill
```

#### 4. Setup udev rules

```text
    - check USB devices:
        => `lsusb | grep ST-LINK`
            Bus 001 Device 002: ID 0483:3748 STMicroelectronics ST-LINK/V2 
            (So the idVendor is 0483 and idProduct is 3748)
    - open this rules file with Vi to write
        => `sudo vi /etc/udev/rules.d/99-openocd.rules`
    - copy paste this into that file:
        => # STM32F3DISCOVERY - ST-LINK/V2.1
           ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE:="0666"
        => [ESC + ":" + "w"] to write
        => [ESC + ":" + "q"] to exit
    - Reload udev rules:
        => `sudo udevadm control --reload-rules`
```

#### 5. Verify permissions

```text
    - Check USB device enumeration:
        => lsusb | grep -i stm
            Bus 001 Device 002: ID 0483:3748 STMicroelectronics ST-LINK/V2
            (This means the file /dev/bus/usb/001/002 is the STM32F4DISCOVERY)
    - Check permissions:
        => $ ls -la /dev/bus/usb/001/002
            crw-rw---- 1 root plugdev 189, 1 Nov 19 02:46 /dev/bus/usb/001/002
            (should be crw-rw-rw- 1 root root instead...)
            (r: read, w: write, x: execute)
            ([c means character device][-3: u-owner][-3: g-group][-3: o-others])
            
            (plugdev: Allows members to mount (only with the options nodev and nosuid, for security reasons) and umount removable devices through pmount.)
    - To modify into crw-rw----:
        => sudo chmod o+rw 002
            crw-rw-rw- 1 root plugdev 189, 1 Nov 19 02:46 002
            (this step persists after detaching)
    - Change owner+group:
        => sudo chown <user>:<group> <file>
        => sudo chown :root 002 (for our purpose)
            crw-rw-rw- 1 root root 189, 1 Nov 19 02:46 002
            (this step does not presists after detaching)
```

#### 6. Verify OCD connection

```text
    - The interface directory is typically located in /usr/share/openocd/scripts/, which is the default location OpenOCD expects these files.
    - If you've installed them somewhere else use the -s /path/to/scripts/ option to specify your install directory.
    
        => openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg (stlink-v2-1.cfg is deprecated)
        => openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
            Open On-Chip Debugger 0.11.0
            Licensed under GNU GPL v2
            For bug reports, read
                    http://openocd.org/doc/doxygen/bugs.html
            Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
            Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
            Info : Listening on port 6666 for tcl connections
            Info : Listening on port 4444 for telnet connections
            Info : clock speed 2000 kHz
            Info : STLINK V2J14S0 (API v2) VID:PID 0483:3748
            Info : Target voltage: 2.892743
            Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
            Info : starting gdb server for stm32f4x.cpu on 3333
            Info : Listening on port 3333 for gdb connections
            (option to open port 3333 in browser)
    - OpenOCD is a service which forwards debug information from the ITM channel to a file, itm.txt, 
      as such it runs forever and does not return to the terminal prompt.
```

#### 7. Fixing Error from rust-analyzer "can't find crate for 'test'"

- Add config to /home/lamteteeow/.vscode/settings.json
=> `"rust-analyzer.check.allTargets": false`
(because This is due to rust-analyzer implicitly passing --all-targets to the flycheck cargo invocation
which makes it include the test crate which depends on std)

#### 8. As a sanity check, let's verify that the produced executable is actually an ARM binary

```text
    => `cargo readobj --target thumbv7em-none-eabihf --bin test-size -- --file-header`
    or `cargo readobj --bin test-size -- --file-header` (in case .cargo/config.toml was tuned beforehand)
        (base) lamteteeow@Lami:~/STM32F4DISCOVERY_Rust/test-size$ cargo readobj --bin test-size -- --file-header
            Finished dev [unoptimized + debuginfo] target(s) in 0.00s
        ELF Header:
          Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
          Class:                             ELF32
          Data:                              2's complement, little endian
          Version:                           1 (current)
          OS/ABI:                            UNIX - System V
          ABI Version:                       0
          Type:                              EXEC (Executable file)
          Machine:                           ARM
          Version:                           0x1
          Entry point address:               0x0
          Start of program headers:          52 (bytes into file)
          Start of section headers:          972976 (bytes into file)
          Flags:                             0x5000400
          Size of this header:               52 (bytes)
          Size of program headers:           32 (bytes)
          Number of program headers:         3
          Size of section headers:           40 (bytes)
          Number of section headers:         15
          Section header string table index: 13
```

#### 9. FLASHHHHH

```text
    - Cd into /tmp folder (default folder understood by *nix system, similar to /home)
        => cd /tmp
    - Launch OpenOCD inside /tmp:
        => `openocd -f interface/stlink.cfg -f target/stm32f4x.cfg`
            (base) lamteteeow@Lami:/tmp$ openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
            Open On-Chip Debugger 0.11.0
            Licensed under GNU GPL v2
            For bug reports, read
                    http://openocd.org/doc/doxygen/bugs.html
            Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
            Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
            Info : Listening on port 6666 for tcl connections
            Info : Listening on port 4444 for telnet connections
            Info : clock speed 2000 kHz
            Info : STLINK V2J14S0 (API v2) VID:PID 0483:3748
            Info : Target voltage: 2.892743
            Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
            Info : starting gdb server for stm32f4x.cpu on 3333
            Info : Listening on port 3333 for gdb connections
    - Do not click VSCode pop-up to open browser on http://127.0.0.1:3333/
    - Leave that openocd process running
    - In a new terminal inside the project's "/home/lamteteeow/STM32F4DISCOVERY_Rust/test-size/" directory, run:
        NOTE:    `gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/test-size` and `arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/test-size` do not work on this machine
        => `gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/test-size`
           [on other terminal] Info : accepting 'gdb' connection on tcp/3333
                               Info : device id = 0x10016413
                               Info : flash size = 1024 kbytes
                               Info : flash size = 512 bytes
                               Warn : Prefer GDB command "target extended-remote 3333" instead of "target remote 3333"
        => `gdb-multiarch -q -ex "target extended-remote :3333" target/thumbv7em-none-eabihf/debug/test-size` (remove Warn)
        => [q + Enter] to stop and quit gdb server, [y + Enter] to confirm

    - Update ../.cargo/config.toml based on prior steps:

        ```toml
        [target.thumbv7em-none-eabihf]
        # runner = "arm-none-eabi-gdb -q -x ../openocd.gdb"
        runner = "gdb-multiarch -q -x ../openocd.gdb"
        # runner = "gdb -q -x ../openocd.gdb"
        rustflags = [
          "-C", "link-arg=-Tlink.x",
        ]
        [build]
        target = "thumbv7em-none-eabihf"
        ```

    - Now we add linker script memory.x to project folder
    - Open OpenOCD in 1 terminal:
        => openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
    - Go to project foler:
        => cargo run (to start debugging)
    - Follow these steps: <https://docs.rust-embedded.org/discovery/f3discovery/05-led-roulette/debug-it.html>
```

#### 10. GDB CLI syntax

```text
    - (gdb) "ENTER" = repeat last command
    - (gdb) target remote :3333
    - (gdb) load (flash the code)
    - (gdb) set print asm-demangle on
    - (gdb) set style sources off
    - (gdb) break main / b main (??? break <fct> ???)
    - (gdb) continue / c
    - (gdb) finish
    - (gdb) disassemble /m
    - (gdb) monitor reset halt
    - (gdb) step / s (step in)
    - (gdb) next / n (step over)
    - (gdb) info locals
    - (gdb) info args
    - (gdb) backtrace / bt
    - (gdb) print <var> / p <var>
    - (gdb) set <var> = <value>
    - (gdb) l <line#>
    - (gdb) f (print current line we are on)
    - (gdb) d (delete all breakpoints)
    - (gdb) stepi / si
    - (gdb) quit / q
    - For better TUI:
        => (gdb) layout src (or asm or split)
        => (gdb) tui disable (to exit TUI)
    - MORE: <https://docs.rust-embedded.org/discovery/f3discovery/appendix/2-how-to-use-gdb/>
```

#### 11. Add-on to GDB CLI : gdb-dashboard -> <https://github.com/cyrus-and/gdb-dashboard#gdb-dashboard>

It uses Python to turn the default GDB CLI into a dashboard that shows registers, the source view, the assembly view and other things.

- `wget -P ~ https://git.io/.gdbinit` (at /home)
- `pip install pygments` (for syntax highlighting)

#### 12. Tips

```text
    - Clear out memory:
        => `cargo clean`
        => `cargo-cache -a` (to clear the ~/.cargo folder, first => cargo install cargo-cache)
        => `cargo-cache --remove-dir all` ()
    - Copy openocd.gdb and openocd.cfg to local project folder (was not installed in project folder):
        => </home/lamteteeow/.cargo/registry/src/index.crates.io-6f17d22bba15001f/stm32f407g-disc-0.4.1/openocd.gdb>
        => </home/lamteteeow/.cargo/registry/src/index.crates.io-6f17d22bba15001f/stm32f407g-disc-0.4.1/openocd.cfg>
      Then set config.toml file:

        ```toml
        [target.thumbv7em-none-eabihf]
        runner = "gdb-multiarch -q -x ./openocd.gdb"
        ```

    - Check binary file size after compile (in release mode):
        => `cargo size --target thumbv7em-none-eabihf --bin main --release -- -A`
```

##### Next doc: </home/lamteteeow/STM32F4DISCOVERY_Rust/test-size/SetupLog3.txt>
