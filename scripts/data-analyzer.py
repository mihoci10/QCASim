import tarfile
import sys
import json
import struct
import matplotlib.pyplot as plt
import numpy as np
import os

DESIGN_MEMBER = 'DESIGN.json'
METADATA_MEMBER = 'METADATA.json'
DATA_MEMBER = 'DATA.bin'

if len(sys.argv) != 2:
    print("First argument needs to be a *.qcs file!")
    sys.exit(1)

file_arg = sys.argv[1]

with tarfile.open(file_arg, 'r') as archive:
    design = None
    metadata = None
    data = None
    try:
        design = archive.getmember(DESIGN_MEMBER)
        metadata = archive.getmember(METADATA_MEMBER)
        data = archive.getmember(DATA_MEMBER)
    except KeyError as e:
        print(f"QCS file is missing an entry: {e}")
        sys.exit(1)

    design_content = archive.extractfile(design).read()
    metadata_content = archive.extractfile(metadata).read()
    data_content = archive.extractfile(data).read()

    design_json = json.loads(design_content)
    metadata_json = json.loads(metadata_content)
    sim_data = data_content

    qca_core_design_ver = design_json['qca_core_version']
    qca_core_sim_ver = metadata_json['qca_core_version']
    sim_model = design_json['simulation_model_settings'][design_json['selected_simulation_model_id']]
    num_samples = sim_model['num_samples']
    sim_cells = metadata_json['stored_cells']

    print(f'QCA Core design version: {qca_core_design_ver}')
    print(f'QCA Core simulation version: {qca_core_sim_ver}')
    print(f'Num samples: {num_samples}')

    sim_data_off = 0

    clock_data = []
    for _ in range(4):
        data = [None] * num_samples
        for i in range(num_samples):
            data[i] = struct.unpack('<d', sim_data[sim_data_off:sim_data_off+8])[0]
            sim_data_off += 8
        clock_data.append(data)

    cell_data = []
    for cell in sim_cells:
        l = cell['layer']
        arch_id = design_json['layers'][l]['cell_architecture_id']
        polarization_count = design_json['cell_architectures'][arch_id]['dot_count'] // 4
        data = [[None] * num_samples for _ in range(polarization_count)]
        for i in range(num_samples):
            for p in range(polarization_count):
                data[p][i] = struct.unpack('<d', sim_data[sim_data_off:sim_data_off+8])[0]
                sim_data_off += 8
        cell_data.append(data)

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
