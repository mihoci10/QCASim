import os
import subprocess
import sys

QCA_SIM = '../target/release/qca-sim'
INPUT_DIR = 'out'


def parse_truth_table(table_raw: str) -> list[list[int]]:
    result = []
    for table_row in table_raw.splitlines()[1:]:
        result.append(list(map(
            lambda v: -1 if v == "NaN" else int(v),
            table_row.split('\t')[:-1]
        )))
    return result


def run_analysis(filename: str, delays: list[str]):
    delay_args = []
    for delay in delays:
        delay_args += ['-d', delay]
    result = subprocess.run([QCA_SIM, 'truth', filename] + delay_args, capture_output=True, text=True)
    parsed_result = parse_truth_table(result.stdout)
    return parsed_result


print(f'Searching for files in: {INPUT_DIR}...')
for file in os.scandir(INPUT_DIR):
    if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcs'):
        continue
    base_name = os.path.splitext(os.path.basename(file.path))[0]
    print(f'Found file: {base_name}')
    table = run_analysis(file.path, sys.argv[2:])
    print(table)
