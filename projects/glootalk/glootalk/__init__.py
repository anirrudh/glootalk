from .glootalk import *
from .nbmodel import *
    
notebookState = {}

def init_nb():
    notebookState.update({ 'nb': init_notebook(), 'shared': automerge.new_backend() })
    return

def get_state():
    print(notebookState)
