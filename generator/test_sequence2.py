import requests

# Find the generator in the registry
projects = requests.get("http://127.0.0.1:7878/project").json()
print("Found projects:", projects)

for project in projects:
    if project["name"].startswith("Jaka & Tinkara"):
        base_url = f"http://{project['ip']}:{project['port']}"
        print(f"Found generator at {base_url}")
        
        # The exact JSON from your example
        body = {
            "range": {
                "from": 0,
                "to": 30,
                "step": 1
            },
            "parameters": [3, 2, 10],
            "sequences": [
                {"name": "Constant", "parameters": [10], "sequences": []},  # Note: using "Constant" instead of "const"
                {"name": "Sum", "parameters": [], "sequences": [           # Note: using "Sum" instead of "sum"
                    {"name": "Arithmetic", "parameters": [5, 10], "sequences": []},  # Note: using "Arithmetic"
                    {"name": "Constant", "parameters": [10], "sequences": []}        # Note: using "Constant"
                ]}
            ]
        }
        
        # Send request to Mix sequence
        response = requests.post(f"{base_url}/sequence/Mix", json=body)
        print("Response status:", response.status_code)
        if response.status_code == 200:
            result = response.json()
            print("\nSequence result:")
            for i, value in enumerate(result):
                print(f"[{i}]: {value}")
        else:
            print("Error:", response.text)
        break
else:
    print("Generator not found")