import os
import pandas as pd
import subprocess
import sys

QCA_SIM = '../target/release/qca-sim'


def parse_truth_table(table_raw: str) -> list[list[str]]:
    result = []
    for table_row in table_raw.splitlines()[1:]:
        result.append(list(map(
            lambda v: 'NaN' if v == "NaN" else str(v),
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


def calculate_table_accuracy(table: list[list[str]], cmp_func):
    if len(table) == 0:
        return 1.0

    accuracy = 0.0
    for row in table:
        row_accuracy = cmp_func(row)
        accuracy += row_accuracy
    return accuracy / len(table)


def cmp_var_line(row: list[str]) -> float:
    in_val = row[0]
    accuracy_arr = [1.0 if in_val == val else 0.0 for val in row[1:]]
    return sum(accuracy_arr) / (len(row) - 1)


def cmp_var_inverter(row: list[str]) -> float:
    in_val = row[0]
    out_val = row[-1]

    accuracy_arr = [1.0 if in_val == val else 0.0 for val in row[1:-1]]

    if in_val == 1.0:
        accuracy_main = 1.0 if out_val == 1.0 else 0.0
    elif in_val == 0.0:
        accuracy_main = 1.0 if out_val == 2.0 else 0.0
    elif in_val == 2.0:
        accuracy_main = 1.0 if out_val == 0.0 else 0.0
    else:
        raise RuntimeError

    return accuracy_main

def cmp_majority(row: list[str]) -> float:
    truth_table = {
        ('A', 'A', 'A'): 'A',  # A A A -> A
        ('A', 'A', 'B'): 'A',  # A A B -> A
        ('A', 'A', 'C'): 'A',  # A A C -> A
        ('A', 'B', 'A'): 'A',  # A B A -> A
        ('A', 'B', 'B'): 'B',  # A B B -> B
        ('A', 'B', 'C'): 'C',  # A B C -> C
        ('A', 'C', 'A'): 'A',  # A C A -> A
        ('A', 'C', 'B'): 'C',  # A C B -> C
        ('A', 'C', 'C'): 'C',  # A C C -> C
        ('B', 'A', 'A'): 'A',  # B A A -> A
        ('B', 'A', 'B'): 'B',  # B A B -> B
        ('B', 'A', 'C'): 'C',  # B A C -> C
        ('B', 'B', 'A'): 'B',  # B B A -> B
        ('B', 'B', 'B'): 'B',  # B B B -> B
        ('B', 'B', 'C'): 'B',  # B B C -> B
        ('B', 'C', 'A'): 'C',  # B C A -> C
        ('B', 'C', 'B'): 'B',  # B C B -> B
        ('B', 'C', 'C'): 'C',  # B C C -> C
        ('C', 'A', 'A'): 'A',  # C A A -> A
        ('C', 'A', 'B'): 'C',  # C A B -> C
        ('C', 'A', 'C'): 'C',  # C A C -> C
        ('C', 'B', 'A'): 'C',  # C B A -> C
        ('C', 'B', 'B'): 'B',  # C B B -> B
        ('C', 'B', 'C'): 'C',  # C B C -> C
        ('C', 'C', 'A'): 'C',  # C C A -> C
        ('C', 'C', 'B'): 'C',  # C C B -> C
        ('C', 'C', 'C'): 'C',  # C C C -> C
    }

    if len(row) != 4:
        raise RuntimeError
    if 'NaN' in row:
        return 0.0

    [x, y, z, r] = row
    if x == 'D':
        x = 'C'
    if y == 'D':
        y = 'C'
    if z == 'D':
        z = 'C'
    if r == 'D':
        r = 'C'

    if (x, y, z) in truth_table:
        if truth_table[(x, y, z)] != r:
            print(f'({x} {y} {z}) -> {truth_table[(x, y, z)]} != {r}')
        return 1.0 if truth_table[(x, y, z)] == r else 0.0
    else:
        raise RuntimeError


x_coords = []
y_coords = []
accuracies = []

if len(sys.argv) <= 1:
    print('Usage: python analyze_truth.py <filename> [<cell_delay>,...]')
    sys.exit(1)

input_dir = sys.argv[1]

print(f'Searching for files in: {input_dir}...')
for file in os.scandir(input_dir):
    if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcs'):
        continue
    base_name = os.path.splitext(os.path.basename(file.path))[0]
    parts = base_name.split('_')
    x = float(parts[-2])
    y = float(parts[-1])
    print(f'Found file: {base_name}')

    table = run_analysis(file.path, sys.argv[2:])
    accuracy = calculate_table_accuracy(table, cmp_majority)

    x_coords.append(x)
    y_coords.append(y)
    accuracies.append(accuracy)

output = f'{input_dir}/truth_analysis.csv'
df = pd.DataFrame({
    'x_coord': x_coords,
    'y_coord': y_coords,
    'accuracy': accuracies
})
df.to_csv(output, index=False)
print(f'Saved truth analysis to: {output}')