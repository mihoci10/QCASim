import tarfile
import sys
import json

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
    data = data_content

    qca_core_design_ver = design_json['qca_core_version']
    qca_core_sim_ver = metadata_json['qca_core_version']
    sim_model = design_json['simulation_model_settings'][design_json['selected_simulation_model_id']]
    num_samples = sim_model['num_samples']

    print(f'QCA Core design version: {qca_core_design_ver}')
    print(f'QCA Core simulation version: {qca_core_sim_ver}')
    print(f'Num samples: {num_samples}')