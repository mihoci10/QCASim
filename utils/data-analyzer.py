import sys
import numpy as np
import struct
import matplotlib.pyplot as plt

filename = "D:\\Projects\\QCASim\\qca-core\\bistable_file_02.bin" #sys.argv[1]

output_count = 0
clocks = [[], [], [], []]
outputs = []
with open(filename, 'rb') as file:
    output_count = int.from_bytes(file.read(8),'little')
    print(f'output cells: {output_count}')
    outputs = [[] for i in range(output_count)]

    b = file.read(8)
    while b:
        clocks[0].append(struct.unpack('<d', b))
        clocks[1].append(struct.unpack('<d', file.read(8)))
        clocks[2].append(struct.unpack('<d', file.read(8)))
        clocks[3].append(struct.unpack('<d', file.read(8)))

        for i in range(output_count):
            outputs[i].append(struct.unpack('<d', file.read(8)))
        b = file.read(8)

x = np.linspace(0, 1000, len(clocks[0]))

for i in range(len(clocks)):
    clocks[i] = np.array(clocks[i])
for i in range(len(outputs)):
    outputs[i] = np.array(outputs[i])

fig, ax1 = plt.subplots()
ax2 = ax1.twinx()

ax1.plot(x, clocks[0], label='Clock 0', color='red')

ax2.plot(x, outputs[0], label='Cell 1')
ax2.plot(x, outputs[1], label='Cell 2')

fig.legend()
fig.set_size_inches(12,4)
fig.tight_layout()
plt.show()