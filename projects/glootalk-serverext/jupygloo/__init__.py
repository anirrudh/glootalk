from .app import jupyglooServer

def _jupyter_server_extension_paths():
    return [{
        "module": "jupygloo.app",
        "app": jupyglooServer
    }]

