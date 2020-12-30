# glootalk

Rust websocket server written as Python Extension with PyO3. 

## Installing

1) `cd` into the folder and run `conda create -f environment.yaml`
2) `conda activate glootalk`
3) `python setup.py install`
4) `cd` out of the directory, and open a pyhton shell.
5) 
   ```python
   >>> import glootalk
   >>> dir(glootalk)
   >>> glootalk.start_server(port=9001, log_fs_path=".")
   ```
# jupygloo

Jupyter server extension that implements the `glootalk` extensions.
