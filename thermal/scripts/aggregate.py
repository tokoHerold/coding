import numpy as np
from sys import argv

# Use ~/.python/venv/bin/python

def aggregate(path : str):

    # tctl = np.Array()
    # cpu = np.Array()

    content = []
    with open(path, "r") as file:
        content = list(file.read().splitlines())
        content = [c.split("\t") for c in content]

    # print(content)

    tctl = np.array([int(c[0]) for c in content])
    cpu = np.array([int(c[1]) for c in content])



    for var, name in [(tctl, "TCTL"), (cpu, "CPU")]:
        print(f"{name}: mean={var.mean()}, median={np.median(var)}, cov={np.cov(var)}")

if __name__ == "__main__":
    if len(argv) < 2:
        print(f"Usage: python {argv[0]} <file>")
        quit(1)

    aggregate(argv[1])
