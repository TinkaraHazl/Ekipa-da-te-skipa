import requests

# Find the generator
projects = requests.get("http://127.0.0.1:7878/project").json()
for project in projects:
    if project["name"].startswith("Jaka & Tinkara"):
        base_url = f"http://{project['ip']}:{project['port']}"
        
        # Complex nested sequence test
        body = {
            "range": {
                "from": 0,
                "to": 20,
                "step": 1
            },
            "parameters": [2, 3],  # Take 2 from first, 3 from second
            "sequences": [
                # First sequence: Sum of Geometric and Arithmetic
                {
                    "name": "Sum",
                    "parameters": [],
                    "sequences": [
                        {
                            "name": "Geometric",
                            "parameters": [2, 3],  # Start at 2, multiply by 3
                            "sequences": []
                        },
                        {
                            "name": "Arithmetic",
                            "parameters": [1, 2],  # Start at 1, add 2
                            "sequences": []
                        }
                    ]
                },
                # Second sequence: Product of two constants
                {
                    "name": "Product",
                    "parameters": [],
                    "sequences": [
                        {
                            "name": "Constant",
                            "parameters": [5],
                            "sequences": []
                        },
                        {
                            "name": "Constant",
                            "parameters": [3],
                            "sequences": []
                        }
                    ]
                }
            ]
        }
        
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