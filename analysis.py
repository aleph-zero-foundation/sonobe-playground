import json
import re

import matplotlib.pyplot as plt
import numpy as np


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
hypernova = [[None] * 7 for _ in range(7)]


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

            hypernova_params = re.fullmatch(r"HyperNova<(\d),(\d)>", folding_scheme)
            if hypernova_params:
                hypernova[int(hypernova_params.groups()[0])][int(hypernova_params.groups()[1])] = time_seconds
        else:
            free_logs.append(report(span["name"], time_seconds))
        return

    # Within a folding scheme scenario
    if scenario_name not in scenarios:
        scenarios[scenario_name] = {
            "Prepare folding": 0,
            "Transform input": 0,
            "Folding verification": 0,
            "Proving": [],
            "Input prep": [],
        }

    span_name = span.get("name")
    if span_name == "Proving":
        scenarios[scenario_name]["Proving"].append(time_seconds)
    elif span_name == "Input prep":
        scenarios[scenario_name]["Input prep"].append(time_seconds)
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

        print(f"  Folding Steps:")
        input_trans = data["Input prep"]
        print("    Input preparation")
        print(report("      Avg", sum(input_trans) / len(input_trans)))
        print(report("      Min", min(input_trans)))
        print(report("      Max", max(input_trans)))
        proving_steps = data["Proving"]
        print("    Proving")
        print(report("      Avg", sum(proving_steps) / len(proving_steps)))
        print(report("      Min", min(proving_steps)))
        print(report("      Max", max(proving_steps)))


def draw_hn_plot():
    data_np = np.array(hypernova, dtype=np.float64)
    data_np = np.where(np.isnan(data_np), 0, data_np)  # Replace None with 0 for better visualization

    cmap = plt.cm.viridis
    cmap.set_under('white')  # Set background color for None

    fig, ax = plt.subplots()
    cax = ax.matshow(data_np, cmap=cmap, vmin=0.01)

    fig.colorbar(cax)

    for i in range(len(hypernova)):
        for j in range(len(hypernova[i])):
            if hypernova[i][j] is not None:
                ax.text(j, i, f'{hypernova[i][j]:.2f}', va='center', ha='center', color='black')

    # Set axis labels and title
    ax.set_xlabel('ν (number of incoming CCCS instances)')
    ax.set_ylabel('μ (number of running LCCCS instances)')

    ax.set_xticks(np.arange(len(hypernova[0])))

    ax.set_yticks(np.arange(len(hypernova)))
    ax.set_xticklabels([f'{i}' for i in range(len(hypernova[0]))])
    ax.set_yticklabels([f'{i}' for i in range(len(hypernova))])

    plt.title("HyperNova multifold times")

    # Show the plot
    plt.show()


process_logs('out.log')
print_results()
draw_hn_plot()
