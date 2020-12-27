### Examples

These following projects are packages that are installable. 


### glootalk-python

Rust websocket server written as Python Extension with PyO3. 

#### Installing

1) `cd` into the folder and run `conda create -f environment.yaml`
2) `conda activate glootalk`
3) `python setup.py install`
4) `cd` out of the directory, and open a pyhton shell.
5) 
   >>> import glootalk
   >>> dir(glootalk)
   >>> glootalk.start_server() # Start ws://127.0.0.1:9001/
   
### rs-py

Call python from rust

### Notes

`cargo` is automatically distributed with rustc. This means that as long as we install the `rust` package from conda-forge, we should be good.
