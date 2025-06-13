import os
import subprocess
import sys

import pandas as pd

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


def calculate_table_accuracy(table: list[list[int]], cmp_func):
    if len(table) == 0:
        return 1.0

    accuracy = 0.0
    for row in table:
        row_accuracy = cmp_func(row)
        accuracy += row_accuracy
    return accuracy / len(table)


def cmp_var_line(row: list[int]) -> float:
    in_val = row[0]
    accuracy_arr = [1.0 if in_val == val else 0.0 for val in row[1:]]
    return sum(accuracy_arr) / (len(row) - 1)


x_coords = []
y_coords = []
accuracies = []

print(f'Searching for files in: {INPUT_DIR}...')
for file in os.scandir(INPUT_DIR):
    if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcs'):
        continue
    base_name = os.path.splitext(os.path.basename(file.path))[0]
    parts = base_name.split('_')
    x = int(parts[-2])
    y = int(parts[-1])
    print(f'Found file: {base_name}')

    table = run_analysis(file.path, sys.argv[2:])
    accuracy = calculate_table_accuracy(table, cmp_var_line)

    x_coords.append(x)
    y_coords.append(y)
    accuracies.append(accuracy)

output = f'{INPUT_DIR}/truth_analysis.csv'
df = pd.DataFrame({
    'x_coord': x_coords,
    'y_coord': y_coords,
    'accuracy': accuracies
})
df.to_csv(output, index=False)
print(f'Saved truth analysis to: {output}')