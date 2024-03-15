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
