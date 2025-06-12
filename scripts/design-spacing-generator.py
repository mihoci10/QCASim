import copy
import json
import math
import os
import sys

OUT_DIR = 'out'


def set_intercell_distance(orig_design: any, side_length: float, spacing: float) -> any:
    result = copy.deepcopy(orig_design)
    for layer in result['layers']:
        for cell in layer['cells']:
            cell['position'] = list(map(
                lambda p: (p/side_length) * (side_length + spacing),
                cell['position']
            ))

    return result


if len(sys.argv) != 2:
    print("First argument needs to be a *.qcd file!")
    sys.exit(1)

with open(sys.argv[1], 'r') as design_file:
    base_name = os.path.splitext(os.path.basename(sys.argv[1]))[0]
    design = json.loads(design_file.read())['design']

    arch_id = design['layers'][0]['cell_architecture_id']
    arch = design['cell_architectures'][arch_id]
    side_length = arch['side_length']
    dot_pos = arch['dot_positions'][0]
    dot_radius = math.sqrt(dot_pos[0] ** 2 + dot_pos[1] ** 2)

    print(f'Architecture:')
    print(f'  Name: {arch['name']}')
    print(f'  Side length: {side_length}nm')
    print(f'  Dot diameter: {arch['dot_diameter']}nm')
    print(f'  Dot pos radius: {dot_radius}nm')

    spacings = [0, 5, 10]
    print(f'Generating spacing {min(spacings)}..{max(spacings)}')
    for spacing in spacings:
        new_design = set_intercell_distance(design, side_length, spacing)
        with open(f'{OUT_DIR}/base_name_{side_length}_{spacing}.qcd', 'w') as new_design_file:
            new_design_file.write(json.dumps({"design": new_design}))
    print(f'Generated designs saved to: {os.path.abspath(OUT_DIR)}')
