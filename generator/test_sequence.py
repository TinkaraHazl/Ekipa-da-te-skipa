import requests

# Print all projects found in registry
print("Checking registry...")
projects = requests.get("http://127.0.0.1:7878/project").json()
print("Found projects:", projects)

for project in projects:
    print(f"Checking project: {project}")
    if project["name"].startswith("Jaka & Tinkara"):
        base_url = f"http://{project['ip']}:{project['port']}"
        print(f"Found generator at {base_url}")
        
        # Test the complex sequence
        body = {
            "range": {
                "from": 0,
                "to": 30,
                "step": 1
            },
            "parameters": [3, 2, 10],
            "sequences": [
                {"name": "Constant", "parameters": [10], "sequences": []},
                {"name": "Sum", "parameters": [], "sequences": [
                    {"name": "Arithmetic", "parameters": [5, 10], "sequences": []},
                    {"name": "Constant", "parameters": [10], "sequences": []}
                ]}
            ]
        }
        
        # Send request to Mix sequence
        response = requests.post(f"{base_url}/sequence/Mix", json=body)
        print("Response status:", response.status_code)
        print("Sequence result:", response.json())
        break
else:
    print("Generator not found")