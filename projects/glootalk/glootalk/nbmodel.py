from .glootalk import *
import nbformat
import jupyter_server

notebookState = {}

def init_notebook():
    nb = nbformat.v4.new_notebook()
    return nb

def read_notebook_fp(path):
    nb = nbformat.read(fp, nbformat.NO_CONVERT)
    return nb

def read_notebook_dict(nbdict):
    nb = nbformat.from_dict(nbdict)
    nbformat.validate(nb)
    return nb
    

