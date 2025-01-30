from sys import argv
from matplotlib import pyplot as plt
import numpy as np

datalines = []
data_names = []

def add_dataline(path : str):
    """
    Adds a dataline to the plot.
    """
    content = []
    with open(path, "r") as file:
        content = list(file.read().splitlines())
        content = [c.split("\t") for c in content]

    # print(content)

    tctl = np.array([int(c[0]) for c in content])
    cpu = np.array([int(c[1]) for c in content])

    datalines.append((tctl, cpu))
    data_names.append(path)

def save_plot(filename = "plot.png"):
    """
    Displays the plot with all added datalines.
    """
    # Create a figure and axis
    fig, ax = plt.subplots(figsize=(10, 6))

    # Loop over all datalines and plot them
    for i, (tctl, cpu) in enumerate(datalines):
        color = np.random.rand(3,)
        ax.plot(tctl, label=f'TCTL {data_names[i]}', color=color)
        ax.set_xlim(0, )

        # Creating strikethrough effect for cpu line
        ax.plot(cpu, label=f'CPU {data_names[i]}', color=color, linestyle='--', linewidth=2, alpha=0.5)
        # for i in range(len(cpu)):
        #     ax.plot([cpu[i], cpu[i]], [cpu[i] + 0.2, cpu[i] - 0.2], color=color, linewidth=2)

    # Add legend to the plot
    ax.legend(loc='center left', bbox_to_anchor=(1.1, 0.5))
    plt.tight_layout(rect=(0.0, 0.0, 0.85, 1.0))

    # Save plot
    plt.savefig(filename)


if __name__ == "__main__":
    if len(argv) < 3:
        print(f"Usage: {argv[0]} <output_name> <data row 1> [data row 2] ...")
        quit(-1)

    for path in argv[2:]:
        add_dataline(path)

    save_plot(argv[1])
