import google.generativeai as genai
from google.ai.generativelanguage import FunctionDeclaration, Schema, Type, Part, FunctionResponse
import proprio
import os
import sys

API_KEY = os.environ.get("GEMINI_API_KEY") or "xxxxxxxxxxxxxxxxxxxxx"

if not API_KEY or "PASTE" in API_KEY:
    print("[!] ERROR: API Key is missing.")
    sys.exit(1)

genai.configure(api_key=API_KEY)

def main():
    print("[*] Connecting to Proprio IPC...")
    try:
        client = proprio.connect("com.test.dummy")
        print(f"[+] Connected to '{client.app_id}'")
        print(f"[+] Tools detected: {[t['name'] for t in client.tools]}")
    except Exception as e:
        print(f"[!] FATAL: Is the Rust app running? Error: {e}")
        return

    # --- 1. SETUP TOOLS ---
    gemini_tools = []
    for t in client.tools:
        props = {}
        for k, v in t["parameters"].items():
            props[k] = Schema(type=Type.INTEGER if v == "int" else Type.STRING)
        
        gemini_tools.append(FunctionDeclaration(
            name=t["name"],
            description=t["description"],
            parameters=Schema(type=Type.OBJECT, properties=props)
        ))

    # --- 2. START SESSION ---
    print("[*] Initializing Gemini 1.5 Flash...")
    model = genai.GenerativeModel('gemini-flash-latest', tools=[gemini_tools])
    chat = model.start_chat() 
    
    print("[+] Agent Ready. Type 'exit' to quit.")
    print("-" * 50)

    # --- 3. MAIN LOOP ---
    while True:
        try:
            user_input = input("\n[USER]> ")
            if user_input.lower() in ["exit", "quit"]: break
        except KeyboardInterrupt:
            break

        print("[*] Thinking...")
        
        try:
            # Send User Input to Gemini
            response = chat.send_message(user_input)
            
            # LOOP: Handle Multi-Step Logic
            # We keep looping as long as the AI keeps asking for function calls
            while response.parts and response.parts[0].function_call:
                fc = response.parts[0].function_call
                tool_name = fc.name
                args = {k: v for k, v in fc.args.items()}
                
                print(f"[LOG] Model requested: {tool_name}({args})")
                
                # A. Execute Tool on Rust
                if hasattr(client, tool_name):
                    func = getattr(client, tool_name)
                    
                    print(f"  [>] Sending to Runtime...")
                    tool_result = func(**args)
                    print(f"  [<] Received: {tool_result}")
                    
                    # B. Send Result Back to Gemini
                    print("  [.] Sending result to Gemini...") # <--- Debug Line
                    
                    response = chat.send_message(
                        Part(function_response=FunctionResponse(
                            name=tool_name,
                            response={"result": tool_result}
                        ))
                    )
                else:
                    print(f"[!] ERROR: Tool '{tool_name}' not found.")
                    break
            
            # Final Text Response
            if response.text:
                print(f"[AI]> {response.text}")

        except Exception as e:
            print(f"[!] ERROR: {e}")

if __name__ == "__main__":
    main()