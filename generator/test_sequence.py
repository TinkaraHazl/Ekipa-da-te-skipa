import requests

print("Checking registry...")
response = requests.get("http://127.0.0.1:7878/project")
print("Response text:", response.text)
projects = response.json()
print("Found projects:", projects)

for project in projects:
    print(f"Checking project: {project}")
    if project["name"].startswith("Jaka & Tinkara"):
        base_url = f"http://{project['ip']}:{project['port']}"
        print(f"Found generator at {base_url}")
        
        # Test with a simple Arithmetic sequence
        body = {
            "range": {
                "from": 0,
                "to": 5,
                "step": 1
            },
            "parameters": [1, 2],  # Start at 1, add 2 each time
            "sequences": []
        }
        
        response = requests.post(f"{base_url}/sequence/Arithmetic", json=body)
        print("Response status:", response.status_code)
        print("Response text:", response.text)
        if response.status_code == 200:
            print("Sequence result:", response.json())
        break
else:
    print("Generator not found")