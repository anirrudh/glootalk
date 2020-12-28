# glootalk-python 

This repository contains the source code for the glootalk python module. The module is written in rust, and PyO3 is used
to create a Python rust-extension.

Current Features:

- [] Working WebSocket Server Implementation

In Progress:

- [] JupyterLab Server Extension

## Environment Setup

#### conda environment

The conda environment contains all the required packages to build the glootalk extension. 

`conda env create -f environment.yaml` will install the `glooktalk` environment.

Activate the `glootalk` environment, which will allow you to build _and_ seperately _install_ the packages.

#### nix environment

## Build

Building the package requires using the `makefile`. `make build` Will build the rust extenion into a python module, located in the target folder. You can import the binary after renaming it to `glootalk.so`


## Install

Simply do a `python setup.py install`

## Usage

```python
>>> from glootalk import start_server
>>> glootalk.start_server()
...
```

The above function starts a WebSocket Server on port `9042`. This runs at localhost, or `127.0.0.1:9042`. You can use any client to send a message whcih will be written to the stdout messages. 
