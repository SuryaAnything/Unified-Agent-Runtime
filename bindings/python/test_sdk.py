import proprio

print("--- Connecting ---")
client = proprio.connect("com.test.dummy")

print("--- Sending Command (Direct Style) ---")

response = client.draw_rectangle(width=50, height=80)

print(f" Result: {response}")

client.close()