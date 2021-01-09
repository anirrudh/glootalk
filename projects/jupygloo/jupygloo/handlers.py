from jupyter_server.base.handlers import JupyterHandler
from jupyter_server.extension.handler import ExtensionHandlerMixin, ExtensionHandlerJinjaMixin
from jupyter_server.base.zmqhandlers import WebSocketMixin

from tornado.websocket import WebSocketHandler, websocket_connect
from tornado.ioloop import IOLoop
import json

import glootalk


class DefaultHandler(ExtensionHandlerMixin, JupyterHandler):
    def get(self):
        # The name of the extension to which this handler is linked.
        self.log.info("Extension Name in {} Default Handler: {}".format(
            self.name, self.name))

        self.write('<h1>Jupygloo Extension</h1>')
        self.write('Config in {} Default Handler: {}'.format(
            self.name, self.config))


shared_automerge_rooms = {}


class AutomergeRoom:

    def __init__(self, doc):

        self.docname = doc
        self.websockets = []
        self.automerge_backend = glootalk.automerge.new_backend()
        print("Room init, document : ", self.automerge_backend)

    def add_websocket(self, ws):
        self.websockets.append(ws)

    def remove_websocket(self, ws):
        self.websockets.remove(ws)

    def get_changes(self):
        return glootalk.automerge.get_changes(self.automerge_backend)

    def dispatch_message(self, message, sender=None):

        # TODO : forward the message to the automerge document
        self.automerge_backend = glootalk.automerge.apply_change(
            self.automerge_backend, message)

        for ws in self.websockets:
            if ws != sender:
                ws.write_message(message, binary=True)


class AutomergeWsHandler(WebSocketMixin, WebSocketHandler, ExtensionHandlerMixin, JupyterHandler):

    async def open(self):

        doc = self.get_argument('doc', default=None)
        print(f"\nDEBUG {self.request}, {self.request.remote_ip}  \n")

        if doc not in shared_automerge_rooms:
            shared_automerge_rooms[doc] = AutomergeRoom(doc)

        shared_automerge_rooms[doc].add_websocket(self)
        print(
            f"\nDEBUG shared websockets for doc {doc} : {shared_automerge_rooms[doc]}")

        print("Websocket open : ",
              shared_automerge_rooms[doc].automerge_backend)

        changes = shared_automerge_rooms[doc].get_changes()
        for c in changes:
            message = bytes(c)
            self.write_message(message, binary=True)

    def on_message(self, message,  *args, **kwargs):

        doc = self.get_argument('doc', default=None)
        if doc not in shared_automerge_rooms:
            print(
                f"WEIRD : on_message for {doc} not in shared_automerge_rooms")
            return

        print(f"\nDEBUG message : {message}")

        shared_automerge_rooms[doc].dispatch_message(message, sender=self)

    def on_close(self,  *args, **kwargs):
        doc = self.get_argument('doc', default=None)

        print(f"WebSocket on close for {doc}")
        if doc in shared_automerge_rooms:
            shared_automerge_rooms[doc].remove_websocket(self)
