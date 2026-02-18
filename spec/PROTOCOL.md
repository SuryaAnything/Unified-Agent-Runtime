# Proprio IPC Protocol (v0.1)

## 1. Overview
Proprio is a local Inter-Process Communication (IPC) standard designed to allow Artificial Intelligence agents to discover, inspect, and control local applications.

## 2. The Transport Layer
* **Linux/macOS:** Unix Domain Sockets (UDS).
* **Windows:** Named Pipes.
* **Encoding:** UTF-8 encoded JSON.

## 3. The Discovery Mechanism (The Registry)
To be visible to an AI Agent, an application MUST place a "Registration File" in the user's home directory:
* **Location:** `~/.proprio/registry/<app_id>.json`
* **Content:**
    ```json
    {
        "app_id": "com.user.texteditor",
        "name": "SuperText",
        "pid": 12345,
        "socket_path": "/tmp/proprio/com.user.texteditor.sock",
        "status": "active"
    }
    ```

## 4. The Message Format
Proprio uses **JSON-RPC 2.0**.

### 4.1. Request (Agent -> App)
```json
{
    "jsonrpc": "2.0",
    "method": "function_name",
    "params": { "arg1": "value" },
    "id": "req_unique_id"
}
```

### 4.2. Response (App -> Agent)
```json
{
    "jsonrpc": "2.0",
    "result": { "status": "ok", "data": "..." },
    "id": "req_unique_id"
}
```

## 5 Reserved Methods
Every Proprio app MUST implement these internal methods:

> __proprio_manifest__: Returns the list of tools/functions the app exposes.
> __proprio_ping__: Returns "pong" (health check). 