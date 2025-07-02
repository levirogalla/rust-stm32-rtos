# Education RTOS Written in Rust

The purpose of this project is to learn about rtos systems, rust, and low level architecture. It uses minimal supporting libraries.

## Running

To run use the cargo tool `cargo embed` by installing it first and then running `cargo embed`. This tool looks for the cpu properties in `Embed.toml`. The tool builds and flashes everything in one step. It also handles setting up various debugging tools such as **rtt** and **gdb**.

## Debugging

To debug, use rtt by importing various functions provided by the `rtt_target` crate. It needs to be enable in `Embed.toml`.

Additional, for more indepth debugging use GDB.

1. Tell cargo embed to enable the gdb debugger in `Embed.toml`
2. Run `cargo embed` to start the code execution.
3. In a new terminal window open gdb by running 
```bash
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/rust-stm32-helloworld
```
4. Connect to the remote target `target remote :1337`
5. Run `monitor reset halt` to restart execution and then `continue` to move between break points

Set breakpoints:
```bash
b src/<file>:<line>
```

```bash
b <symbol>
```

View break points:
```bash
info b
```

View registers:
```bash
info reg
```

View stack:
```bash
x/<n>xw <address>
```
where `n` is the number blocks (4 bytes) to show.