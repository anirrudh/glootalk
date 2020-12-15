# Rust Server for Generic RTC Backend Implementation

This is a research project to create a semi-generic rust deispatch engine alongside [Y.js](https://github.com/yjs/yjs) and [Automerge](https://github.com/automerge/automerge). 

The current implementations for each library with a react frontend can be found at:

* [collarboredit - Automerge implementation](https://github.com/anirrudh/collaborative-edit)
* [proto-rtc-yjs - yjs implementation](https://github.com/pierrotsmnrd/proto-rtc-yjs)

There are currently rust implementations in both of these languages for their server backends.

* [automerge-rs](https://github.com/automerge/automerge-rs)
* [Yrs](https://github.com/yjs/yrs)

## Architecture Overview

We will be using [PyO3](https://github.com/PyO3/) to generate a native python module that will be using a rust websocket server in phase 1. This will be simple enough to handle the requirements for a basic rtc server. 

The websocket server will also run akin to the automerge/rust server. Furthermore, WASM compilation is something that we can also look into. 

##### Packaging

Environment management and packaging will take place using either `conda` or `nix`. 

The rust environment will use [setuptools-rust](https://github.com/PyO3/setuptools-rust) in order to package itself. An [example](https://github.com/PyO3/setuptools-rust/tree/master/examples/html-py-ever) can be found here. 

The server will allow us to properly communitcate with the tornado spun up.
