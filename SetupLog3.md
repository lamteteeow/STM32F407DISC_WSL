
#### Tips

- enums & struct have different accessiblity with "pub". "pub" applies to all enums' children while struct not.
- directly using "use" applies for the same scope only.
- enums is an example of Rust supporting Algebraic data types
- "mod" is different than "include" from other languages. It is more "bidirectional" with the help of Cargo.
- Layout of HAL in Rust:

```text
                                         
                                         _ Micro-architecture Crate -> Microprocessor (ARM Cortex M4) => (cortex-m)
                                        /
                                       /
            Board Crate -> HAL Crate -{                            [STM32F407VG]
                                       \
                                        \_
                                           Peripheral Access Crate (PAC) -> ADC, GPIO, Memory, I2C, SPI, USB, ... => (stm32f407g_disc)
```

- By default: cargo build will create bin in <project>/target/<target>/debug [first target is emulator system, here is Linux(Ubuntu WSL), not Windows]
- By default: cargo build --manifest-path <path>, Cargo searches for the Cargo.toml file in the current directory or any PARENTdirectory.
- Some important options: --target, --release, --examples/--example <example>, --tests/--test <test>, --bins/--bin <bin>, --lib
- To quit GDB without having to confirm everytime, add this file to home directory

```console
$ cat ~/.gdbinit
define hook-quit
    set confirm off
end
```

- Learning macro_rules! lmao {} designators:

```text
block
expr (is used for expressions)
ident (is used for variable/function names)
item
literal (is used for literal constants)
pat (pattern)
path
stmt (statement)
tt (token tree)
ty (type)
vis (visibility qualifier)
```

- Learning macro_rules! lmao {} operators:

```text
$
=>
+ (repeat at least once)
* (repeat any amount of times)
```

- panic strategy can be:
    tagged: `#[cfg(panic = "unwind")]` and `#[cfg(not(panic="unwind"))]`
    or explicit: `if cfg!(panic="abort") {} else {}`
- panic strategy can be set from the command line by using "abort" or "unwind": `rustc  lemo nade.rs -C panic=<panic_strategy>`
- Trait Debug:
    #[derive(Debug)]: automatically implemented for types
    {:?} formatting specifier
    {:#?} pretty-lining
- Closure a.k.a Lambda fn syntax:
    = |var| expr;
    = |var| -> type {expr}(input_para);
    = |var: type| -> type {expr};
    = |var: type| -> type {expr}(input_para);

#### Check list for debugging methods

I have been using OpenOCD remote server to manage debug session through ST-Link (SWD) debugger to the board

```text
    [x] GDB client server => step-through debugging
        - Debug view: 
            [x] Terminal
            [x] ITM (Instrumentation Trace Macrocell) Data console => one-way (from Microcontroller to Host only)

    [ ] Semihosting (cortex-m-semihosting) => printf style 
    [x] probe-rs + deferred format defmt + Real-time transfer RTT
```

The ITM communication protocol works with frames (~ Ethernet frames). Each frame has a header and a variable length payload.
OpenOCD will receive these frames and write them directly to a file without parsing them.
So, if the microntroller sends the string "Hello, world!" using the iprintln macro, OpenOCD's output file won't exactly contain that string.

**NOTE**: It's very important that both "itmdump" and "openocd" are running from the same directory!
    - `cd /tmp && touch itm.txt`
    - `itmdump -F -f itm.txt`

In another terminal:
    - `cd /tmp`
    - `openocd -f interface/stlink.cfg -f target/stm32f4x.cfg`

Then:
    - `cargo run --bin hello_itm`
    + Does not print out anything , both iprint! and panic!, even with connecting a female to female jumper wire between SWO and PB3
    + Assumption: does not seem to work at /tmp => Also does not work in project directory
    + On Github rust-embed/cortex-m issue #229: <https://github.com/rust-embedded/cortex-m/issues/229>
        Update:
                I tested the above example program on another board(Nucleo-429ZI), and it also exhibits the same behavior 😿
                So far I've tested two devices and they both exhibit the same behavior. Both belong to the STM32F4 series.
                    stm32f407g-disc1
                    Nucleo-F429ZI
                At the moment, I'm not sure what's causing this. (guess: issue with OpenOCD?)

>[!danger] => Skip to Registry
    (GPIO): General Purpose Input/Output
    RCC: Reset and Clock Control
