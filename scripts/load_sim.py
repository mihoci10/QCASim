import json
import struct
import sys
import tarfile

DESIGN_MEMBER = 'DESIGN.json'
METADATA_MEMBER = 'METADATA.json'
DATA_MEMBER = 'DATA.bin'


def load_sim_file(filename: str) -> (object, object, list[list[float]], list[list[list[float]]]):
    with tarfile.open(filename, 'r') as archive:
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
        
        sim_settings = design_json['simulation_settings']
        sim_model = sim_settings['simulation_model_settings'][sim_settings['selected_simulation_model_id']]
        num_samples = metadata_json['num_samples']
        sim_cells = metadata_json['stored_cells']

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

        return (design_json, metadata_json, clock_data, cell_data)
