import os
import matplotlib.pyplot as plt

def average(lst):
    return sum(lst) / len(lst)


def create_plot(y_values, y_label, filename, color='tab:blue'):
    plt.figure()
    plt.plot(range(512), y_values, label=y_label, color=color)
    plt.xlabel('The KASLR slots')
    plt.ylabel(y_label)
    # plt.xticks(range(0, 512, 64))
    plt.xticks(list(range(0, 512, 64)) + [511])
    plt.savefig(filename)

path = "thermalbleed"

max_values = []
avg_values = []

# Read files and calculate average and max
for i in range(512):
    filename = os.path.join(path, f"data{i}.txt")
    with open(filename, 'r') as file:
        numbers = [int(line.strip()) // 10000 for line in file]

        avg_value = average(numbers)
        max_value = max(numbers)

        avg_values.append(avg_value)
        max_values.append(max_value)

create_plot(max_values, "Max-Temperature(°C)", "max_values.pdf")
create_plot(avg_values, "Avg-Temperature(°C)", "avg_values.pdf", "tab:orange")
