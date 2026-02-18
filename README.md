# Project Amber : Unified Agent Runtime (UAR)

Neuro-Symbolic AI Agent Runtime
Bridging the reasoning of LLMs with the safety and speed of Rust.


## 1. The Concept
Amber is a proof-of-concept architecture that gives a Large Language Model (LLM) a "physical" body. Unlike standard agents that simply output text, Amber connects a Python-based Brain to a Rust-based High-Performance Runtime via local IPC (Unix Domain Sockets).

This allows the AI to:

Reason about complex tasks (e.g., "Draw a square 10% of the screen width").

Investigate its environment (e.g., call get_screen_size).

Execute type-safe, compiled code on the host machine.

This allows the AI to:
1.  Reason about complex tasks (e.g., "Draw a square 10% of the screen width").
2.  Investigate its environment (e.g., call `get_screen_size`).
3.  Execute type-safe, compiled code on the host machine.

## 2. Proof of Agency (Logs)
Actual logs from v0.1-alpha test run:

```
[USER]> Draw a square that is 10% of the screen width.

[*] Thinking...
[LOG] Model requested: get_screen_size({})
  [>] Sending to Runtime...
  [<] Received: {'height': 1080, 'unit': 'pixels', 'width': 1920}
  [.] Sending result to Gemini...

[LOG] Model requested: draw_rectangle({'height': 192.0, 'width': 192.0})
  [>] Sending to Runtime...
  [<] Received: {'area': 36864, 'status': 'success'}
  [.] Sending result to Gemini...

[AI]> OK. I have drawn a 192x192 pixel square, which is 10% of the screen width.
```
## 3. Architecture

* **Cortex (Brain):** Python / Google Gemini. Handles logic, planning, and tool selection.
* **Nerve (Bridge):** JSON-RPC / Unix Socket. Provides low-latency IPC transport.
* **Exoskeleton (Body):** Rust / Tokio. Handles type-safe execution and resource management.

## 4. Quick Start

### Prerequisites
* Rust (1.75+)
* Python (3.10+)
* Google Gemini API Key

### Step 1: Build the Exoskeleton (Rust)
The "Body" must be running to accept commands.

cd examples/dummy_app
cargo build --release
cargo run
# Output: [+] Listening on /tmp/dummy.sock...
```
[*] Starting Dummy App (IPC Server)...
Registered app: com.test.dummy
[+] Server ready.
[+] Listening on /tmp/dummy.sock...
👂 Listening on /tmp/dummy.sock
📩 Received: {"jsonrpc": "2.0", "method": "__proprio_manifest__", "params": {}, "id": 1}
📩 Received: {"jsonrpc": "2.0", "method": "get_screen_size", "params": {}, "id": 1}
[CMD] get_screen_size called
📩 Received: {"jsonrpc": "2.0", "method": "draw_rectangle", "params": {"height": 192.0, "width": 192.0}, "id": 1}
[CMD] draw_rectangle called with 192x192
```

### Step 2: Wake the Cortex (Python)
Open a new terminal.

```
cd bindings/python
python3 -m venv .venv
source .venv/bin/activate
pip install google-generativeai proprio
```

# Set your Key (Avoid hardcoding!)
export GEMINI_API_KEY="AIzaSy..."

# Run the Agent
```
python real_agent.py
```

## 5. Project Structure

```
Project-Amber/
├── core/                 # Shared Rust libraries (IPC, Router)
├── examples/
│   └── dummy_app/        # The Rust Runtime (The Body)
└── bindings/
    └── python/           # The AI Agent (The Brain)
```