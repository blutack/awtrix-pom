# awtrix-pom
Standalone pomodoro timer for clocks flashed with the awesome [awtrix-light](https://github.com/Blueforcer/awtrix-light) firmware.

![awtrix](https://github.com/blutack/awtrix-pom/assets/348305/bd46a804-1b19-4e33-9174-57c38ae9f5a4)
![awtrix(1)](https://github.com/blutack/awtrix-pom/assets/348305/b8a3f63c-9f22-4c05-bbb8-a36470a9ec56)
![awtrix(2)](https://github.com/blutack/awtrix-pom/assets/348305/f9764e21-51dd-4096-9699-126ce7003b9c)

## Installation & Usage
The recommended way to install is to build it yourself using cargo (after skimming the code to make sure it's not doing anything nefarious) but there are also Github CI build binaries if you like.

### From Binaries
- Download a suitable build zip for your operating system from https://github.com/blutack/awtrix-pom/releases
- Unzip it to find an awtrix-pom or awtrix-pom.exe binary
- Open the folder in a command prompt/terminal window (Windows users, right click in the folder and select "Open command window here")
- Run `awtrix-pom.exe http://my-clock.local` on Windows or `./awtrix-pom http://my-clock.local` on Linux/OSX
- Your clock should switch to a pomodoro timer layout

### From Source Code/Cargo
- Install Rust from https://www.rust-lang.org/tools/install if you don't already have it
- Clone this repo with `git clone https://github.com/blutack/awtrix-pom`
- Change directory into the cloned repository and run `cargo build --release`. Your binary will be output to the `/target/release` folder
- You can also run awtrix-pom directly with `cargo run -- http://my-clock.local`
- Your clock should switch to a pomodoro timer layout

## Options
```
Usage: awtrix-pom <hostname> [--work <work>] [--short <short>] [--long <long>] [--cycles <cycles>] [--beep <beep>] [--switch <switch>] [-v] [--seconds]

A pomodoro timer for awtrix-light.

Positional Arguments:
  hostname          base URL of an awtrix-light instance (http://my-clock.local
                    or http://192.168.2.3)

Options:
  --work            minutes to work for [25]
  --short           minutes to rest for on short breaks [5]
  --long            minutes to rest for on long breaks [20]
  --cycles          how many short breaks before a long break [4]
  --beep            beep on transition between work and breaks [true]
  --switch          switch to the timer app on the clock automatically [true]
  -v, --verbose     print debug information
  --seconds         interpret times as seconds, not minutes - useful for demos
  --help            display usage information

```
