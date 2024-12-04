import requests

def test_sequence(base_url, sequence_type, body):
    print(f"\nTesting {sequence_type} sequence:")
    print("Request body:", body)
    
    response = requests.post(f"{base_url}/sequence/{sequence_type}", json=body)
    print("Response status:", response.status_code)
    print("Response text:", response.text)
    if response.status_code == 200:
        print("Sequence result:", response.json())
    return response.status_code == 200

# Find the generator
response = requests.get("http://127.0.0.1:7878/project")
projects = response.json()
generator = next((p for p in projects if p["name"].startswith("Jaka & Tinkara")), None)

if generator:
    base_url = f"http://{generator['ip']}:{generator['port']}"
    print(f"Found generator at {base_url}")
    
    # Advanced test cases
    test_cases = [
        ("Tribonacci", {
            "range": {"from": 0, "to": 7, "step": 1},
            "parameters": [1, 1, 1],  # First three numbers
            "sequences": []
        }),
        ("Sum", {
            "range": {"from": 0, "to": 5, "step": 1},
            "parameters": [],
            "sequences": [
                {"name": "Arithmetic", "parameters": [1, 2], "sequences": []},
                {"name": "Constant", "parameters": [10], "sequences": []}
            ]
        }),
        ("Drop", {
            "range": {"from": 0, "to": 10, "step": 1},
            "parameters": [3],  # Drop first 3 numbers
            "sequences": [
                {"name": "Arithmetic", "parameters": [1, 1], "sequences": []}
            ]
        })
    ]
    
    for sequence_type, body in test_cases:
        test_sequence(base_url, sequence_type, body)
else:
    print("Generator not found")