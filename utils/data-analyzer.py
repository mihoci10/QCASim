import sys
import numpy as np
import struct
import matplotlib.pyplot as plt

# filename = "D:\\MY PROJECTS\\QCASim\\qca-core\\full_basis_line.bin" #sys.argv[1]
# filename = 'majority.bin'
if len(sys.argv) < 2:
    print('Error, missing argument')
    exit(-1)
filename = sys.argv[1]

output_count = 0
polarity_count = 0
clocks = [[], [], [], []]
outputs = []
with open(filename, 'rb') as file:
    output_count = int.from_bytes(file.read(8),'little')
    polarity_count = int.from_bytes(file.read(8),'little')
    print(f'output cells: {output_count}')
    print(f'polarity count: {polarity_count}')
    outputs = [[[] for p in range(polarity_count)] for i in range(output_count)]

    b = file.read(8)
    while b:
        clocks[0].append(struct.unpack('<d', b))
        clocks[1].append(struct.unpack('<d', file.read(8)))
        clocks[2].append(struct.unpack('<d', file.read(8)))
        clocks[3].append(struct.unpack('<d', file.read(8)))

        for i in range(output_count):
            for p in range(polarity_count):
                outputs[i][p].append(struct.unpack('<d', file.read(8)))
        b = file.read(8)

x = np.linspace(0, 1000, len(clocks[0]))

for i in range(len(clocks)):
    clocks[i] = np.array(clocks[i])
for i in range(len(outputs)):
    for p in range(polarity_count):
        outputs[i][p] = np.array(outputs[i][p])

fig, axs = plt.subplots(len(clocks) + len(outputs))

for i in range(len(clocks)):
    axs[i].plot(x, clocks[i])
    axs[i].set_title(f'Clock {i+1}')

for i in range(len(outputs)):
    for p in range(polarity_count):
        axs[len(clocks) + i].plot(x, outputs[i][p], label='{p}')
    axs[len(clocks) + i].set_title(f'Cell {i+1}')

fig.set_size_inches(10,15)
fig.tight_layout()
fig.savefig(f'Report_{filename.split(".")[0]}.pdf')