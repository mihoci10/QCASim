import matplotlib.pyplot as plt
import numpy as np
import os
import sys

from load_sim import load_sim_file

DESIGN_MEMBER = 'DESIGN.json'
METADATA_MEMBER = 'METADATA.json'
DATA_MEMBER = 'DATA.bin'

if len(sys.argv) != 2:
    print("First argument needs to be a *.qcs file!")
    sys.exit(1)

file_arg = sys.argv[1]

(design, metadata, clock_data, cell_data) = load_sim_file(file_arg)

qca_core_design_ver = design['qca_core_version']
qca_core_sim_ver = metadata['qca_core_version']
sim_model = design['simulation_model_settings'][design['selected_simulation_model_id']]
num_samples = metadata['num_samples']
sim_cells = metadata['stored_cells']

print(f'QCA Core design version: {qca_core_design_ver}')
print(f'QCA Core simulation version: {qca_core_sim_ver}')
print(f'Num samples: {num_samples}')

report_name = os.path.splitext(os.path.basename(file_arg))[0]

fig, axs = plt.subplots(len(clock_data) + len(sim_cells))
sample_axis = np.arange(num_samples)

for i in range(len(clock_data)):
    axs[i].plot(sample_axis, clock_data[i])
    axs[i].set_title(f'Clock (phase {i*90}°)')

for i in range(len(clock_data), len(axs)):
    curr_cell = sim_cells[i-len(clock_data)]
    curr_data = cell_data[i-len(clock_data)]
    for data_p in curr_data:
        axs[i].plot(sample_axis, data_p)
    axs[i].set_title(f'Cell ({curr_cell['layer']},{curr_cell['cell']})')


fig.set_size_inches(10,15)
fig.tight_layout()
fig.savefig(f'report-{report_name}.pdf')
