# A Rust Yew Wasm Project 

This is the front end for the Palabras back end code. They communicate via [GraphQL](graphql/schema.graphql)

The project is in its early stages still. Many of the fundamentals in place. It runs amazingly fast!


## Setup
### Install WebAssembly target
> rustup target add wasm32-unknown-unknown

### Install Rust Trunk Dev Server
install https://github.com/trunk-rs/trunk

### Install the trunk libs
note that this might take a while to install because it compiles everything from scratch
Trunk also provides prebuilt binaries for a number of major package managers
See https://trunkrs.dev/#install for further details
> cargo install --locked trunk


## Run with Trunk in development mode
> trunk serve --open

_Note: remove the --open to not open the browser on launch_

# Testing

**Testing Web Assembly is Non-trivial**

Be sure you have wasm-pack installed
> cargo install wasm-pack
> cargo install wasm-bindgen-cli --vers "0.2.92"

_the --vers must match the version of wasm-bindgen that you 
already have in Cargo.toml!

### Run the tests that require node
> wasm-pack test --node

### Run tests that need a browser

The [webdriver.json](webdriver.json) has some simple default setting. It is required
to be present.

This requires the chrome test driver to be on your path.

Here's an example
[chromedriver-mac-arm64.zip](https://storage.googleapis.com/chrome-for-testing-public/122.0.6261.128/mac-arm64/chromedriver-mac-arm64.zip)
> wasm-pack test --chrome --headless

To test with FireFox, you will need the one of the Gecko drivers:
[geckodrivers](https://github.com/mozilla/geckodriver/releases)
> wasm-pack test --firefox

TODO: need to sort out FireFox testing further_

### reference materials
[Rust Wasm](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/browsers.html)

The other chrome drivers
[Chrome Drivers](https://googlechromelabs.github.io/chrome-for-testing/#stable)

[Chromium CLI Switches](https://peter.sh/experiments/chromium-command-line-switches/)

_be sure to add any drivers to your path_

## Exploring the Application
Once the application is running, you can explore its functionalities, which are rendered entirely within the browser using 
WebAssembly. The project showcases the integration between a Rust-powered front end and a GraphQL back end, demonstrating 
the capabilities and performance benefits of using Rust in web development.

## GraphQL Schema
The communication between the front end and the back end is structured around a GraphQL [schema](graphql/schema.graphql),
which defines the types and queries used to exchange data.

## Future Development
This project is actively being developed. Future updates will focus on expanding its features, improving performance, 
and enhancing the user experience. Contributions and feedback are welcome.
