import socket
import json
import os
import struct

class ProprioClient:
    def __init__(self, app_id):
        self.app_id = app_id
        self.socket_path = self._discover_socket(app_id)
        self.sock = None
        self.tools = []
        self._connect()
        self._fetch_manifest()

    def _discover_socket(self, app_id):
        """Finds the socket path by looking in the registry."""
        home = os.path.expanduser("~")
        registry_path = os.path.join(home, ".proprio", "registry", f"{app_id}.json")
        
        if not os.path.exists(registry_path):
            raise FileNotFoundError(f"App '{app_id}' not found. Is it running?")
            
        with open(registry_path, "r") as f:
            data = json.load(f)
            return data["socket_path"]

    def _connect(self):
        self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        try:
            self.sock.connect(self.socket_path)
            # print(f"✅ Connected to {self.app_id}") <--- Remove this print (we will print better info later)
        except ConnectionRefusedError:
            raise ConnectionError(f"Could not connect to {self.socket_path}.")

    def _fetch_manifest(self):
        """Asks the app for its skills immediately after connecting."""
        try:
            # We manually call the manifest command
            response = self.call("__proprio_manifest__")
            
            # The server returns: { "tools": [...] }
            self.tools = response.get("tools", [])
            
            print(f"✅ Connected to {self.app_id}")
            print(f"📚 Available Tools ({len(self.tools)}):")
            for t in self.tools:
                print(f"   - {t['name']}: {t['description']}")
                
        except Exception as e:
            print(f"⚠️  Warning: Could not fetch manifest: {e}")

    def call(self, method, **params):
        """Sends a JSON-RPC command and waits for the result."""
        request = {
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }
        
        # Send
        payload = json.dumps(request).encode()
        self.sock.sendall(payload)
        
        # Receive (Simple implementation for now)
        response_bytes = self.sock.recv(4096)
        response = json.loads(response_bytes.decode())
        
        if "error" in response and response["error"]:
            raise Exception(f"RPC Error: {response['error']}")
            
        return response["result"]

    def close(self):
        if self.sock:
            self.sock.close()

    def __getattr__(self, name):
        """
        Magic method: if the user calls client.draw_rectangle(), 
        this catches it and redirects it to self.call("draw_rectangle")
        """
        def dynamic_method(**kwargs):
            return self.call(name, **kwargs)
        
        return dynamic_method