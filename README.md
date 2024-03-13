## Usage

install https://github.com/trunk-rs/trunk


## To start this project
> trunk serve


## Setup
### Install WebAssembly target
> rustup target add wasm32-unknown-unknown

### Install the trunk
note that this might take a while to install because it compiles everything from scratch
Trunk also provides prebuilt binaries for a number of major package managers
See https://trunkrs.dev/#install for further details
> cargo install --locked trunk
> 

## Run with Trunk
> trunk serve --open

_Note: remove the --open to not open the browser on launch_
