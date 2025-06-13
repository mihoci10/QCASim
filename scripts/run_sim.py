import os
import subprocess

QCA_SIM = '../target/release/qca-sim'
INPUT_DIR = 'out'


def run_simulation(filename: str, output: str):
    subprocess.run([QCA_SIM, 'sim', filename, '-o', output])


print(f'Searching for files in: {INPUT_DIR}...')
for file in os.scandir(INPUT_DIR):
    if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcd'):
        continue
    base_name = os.path.splitext(os.path.basename(file.path))[0]
    print(f'Found file: {base_name}')
    run_simulation(file.path, f'{INPUT_DIR}/{base_name}.qcs')
