import google.generativeai as genai
import os

# PASTE YOUR KEY HERE
genai.configure(api_key="AIzaSyAfR7bNaSSZDupoRzJPfoV9T-J1LSpmnl4")

print("🔍 Checking available models for your key...")
try:
    for m in genai.list_models():
        if 'generateContent' in m.supported_generation_methods:
            print(f" - {m.name}")
except Exception as e:
    print(f" Error: {e}")