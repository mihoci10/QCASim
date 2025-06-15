import copy
import json
import math
import os
import sys
import numpy as np


def set_intercell_distance(orig_design: any, original_side_length: float, side_length: float, radius: float) -> any:
    result = copy.deepcopy(orig_design)

    for layer in result['layers']:
        for cell in layer['cells']:
            cell['position'] = list(map(
                lambda p: (p/original_side_length) * side_length,
                cell['position']
            ))

    arch_id = result['layers'][0]['cell_architecture_id']
    new_pos = []
    for pos in result['cell_architectures'][arch_id]['dot_positions']:
        [x, y] = pos
        orig_radius =math.sqrt(x**2 + y**2)
        fac = radius / orig_radius
        new_pos.append([x * fac, y * fac])

    result['cell_architectures'][arch_id]['dot_positions'] = new_pos
    return result


if len(sys.argv) != 5:
    print("First argument needs to be a *.qcd file!")
    print("Second argument needs to be a *.qcd file!")
    print("Third argument needs to be intercell distance range <start:stop:step>")
    print("Fourth argument needs to be quantum dot radius range <start:stop:step>")
    sys.exit(1)

filename = sys.argv[1]
output_dir = sys.argv[2]
dist_range_arg = list(map(float, sys.argv[3].split(':')))
radius_range_arg = list(map(float, sys.argv[4].split(':')))

with open(filename, 'r') as design_file:
    base_name = os.path.splitext(os.path.basename(filename))[0]
    design = json.loads(design_file.read())['design']

    spacings = np.arange(dist_range_arg[0], dist_range_arg[1] + dist_range_arg[2], dist_range_arg[2])
    print(f'Generating spacing {min(spacings)} .. {max(spacings)}')

    radiuses = np.arange(radius_range_arg[0], radius_range_arg[1] + radius_range_arg[2], radius_range_arg[2])
    print(f'Generating radius {min(radiuses)} .. {max(radiuses)}')

    architectures = design['cell_architectures']
    arch_id = design['layers'][0]['cell_architecture_id']
    architecture = architectures[arch_id]

    original_side_length = architecture['side_length']

    for side_length in spacings:
        for radius in radiuses:
            new_design = set_intercell_distance(design, original_side_length, side_length, radius)
            with open(f'{output_dir}/{base_name}_{side_length}_{round(radius, 2)}.qcd', 'w') as new_design_file:
                new_design_file.write(json.dumps({"design": new_design}))


    print(f'Generated designs saved to: {os.path.abspath(output_dir)}')
