import json


# Function to convert time units to seconds
def convert_to_seconds(time_str):
    if time_str.endswith('µs'):
        return float(time_str[:-2]) * 1e-6
    elif time_str.endswith('ms'):
        return float(time_str[:-2]) * 1e-3
    elif time_str.endswith('s'):
        return float(time_str[:-1])
    return 0


def report(operation, time):
    return f"{operation}: {time:.6f} seconds"


scenarios = {}
free_logs = []


def process_logline(log):
    fields = log.get("fields", {})
    span = log.get("span", {})
    scenario_name = None
    time_seconds = convert_to_seconds(fields["time.busy"])

    # A folding scheme scenario is one of our ancestors
    for s in log.get("spans", []):
        if s.get("name") == "scenario":
            scenario_name = s.get("folding_scheme")

    # Top level span
    if not scenario_name:
        folding_scheme = span.get("folding_scheme")
        if folding_scheme is not None:
            free_logs.append(report(f"{folding_scheme} total time", time_seconds))
        else:
            free_logs.append(report(span["name"], time_seconds))
        return

    # Within a folding scheme scenario
    if scenario_name not in scenarios:
        scenarios[scenario_name] = {
            "Prepare folding": 0,
            "Transform input": 0,
            "Folding verification": 0,
            "Folding steps": []
        }

    span_name = span.get("name")
    if span_name == "Folding step":
        scenarios[scenario_name]["Folding steps"].append(time_seconds)
    else:
        scenarios[scenario_name][span_name] = time_seconds


def process_logs(file_path):
    with open(file_path, 'r') as f:
        for line in f:
            process_logline(json.loads(line))


def print_results():
    for log in free_logs:
        print(log)
    print()

    for scenario_name, data in scenarios.items():
        print("-" * 80)
        print(f"Scenario: {scenario_name}")
        print(report("  Prepare folding", data["Prepare folding"]))
        print(report("  Transform input", data["Transform input"]))
        print(report("  Folding verification", data["Folding verification"]))

        folding_steps = data["Folding steps"]
        print(f"  Folding Steps:")
        print(report("    Average", sum(folding_steps) / len(folding_steps)))
        print(report("    Min", min(folding_steps)))
        print(report("    Max", max(folding_steps)))


process_logs('out.log')
print_results()
