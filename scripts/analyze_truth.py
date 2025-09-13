from enum import Enum
import os
import subprocess
import sys

import pandas as pd

QCA_SIM = '../target/release/qca-sim'


def _parse_truth_table(table_raw: str) -> list[list[str]]:
    result = []
    for table_row in table_raw.splitlines()[1:]:
        result.append(list(map(
            lambda v: 'NaN' if v == "NaN" else str(v),
            table_row.split('\t')[:-1]
        )))
    return result


def _run_analysis(filename: str, delays: list[str]):
    delay_args = []
    for delay in delays:
        delay_args += ['-d', delay]
    result = subprocess.run([QCA_SIM, 'truth', filename] + delay_args, capture_output=True, text=True)
    parsed_result = _parse_truth_table(result.stdout)
    return parsed_result


def _calculate_table_accuracy(table: list[list[str]], cmp_func):
    if len(table) == 0:
        return 1.0

    accuracy = 0.0
    for row in table:
        row_accuracy = cmp_func(row)
        accuracy += row_accuracy
    return accuracy / len(table)


def _equivariance(val: str) -> str:
    if val == 'D':
        return 'C'
    return val

def _cmp_var_line(row: list[str]) -> float:
    in_val = row[0]
    accuracy_arr = [1.0 if _equivariance(in_val) == _equivariance(val) else 0.0 for val in row[1:]]
    return sum(accuracy_arr) / (len(row) - 1)


def _cmp_inverter(row: list[str]) -> float:
    in_val = row[0]
    out_val = row[-1]

    if _equivariance(in_val) == 'C':
        accuracy_main = 1.0 if _equivariance(out_val) == 'C' else 0.0
    elif _equivariance(in_val) == 'B':
        accuracy_main = 1.0 if _equivariance(out_val) == 'A' else 0.0
    elif _equivariance(in_val) == 'A':
        accuracy_main = 1.0 if _equivariance(out_val) == 'B' else 0.0
    else:
        raise RuntimeError

    return accuracy_main

def _cmp_majority(row: list[str]) -> float:
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

    [x, y, z, r] = [_equivariance(r) for r in row]

    if (x, y, z) in truth_table:
        return 1.0 if truth_table[(x, y, z)] == r else 0.0
    else:
        raise RuntimeError

def _cmp_memory_cell(row: list[str]) -> float:
    truth_table = {
        ('A', 'A'): 'A',
        ('A', 'B'): 'A',
        ('A', 'C'): 'A',
        ('B', 'A'): 'B',
        ('B', 'B'): 'B',
        ('B', 'C'): 'B',
        ('C', 'A'): 'C',
        ('C', 'B'): 'C',
        ('C', 'C'): 'C',
    }

    [x, w, q] = [_equivariance(r) for r in row]
    if (w, x) in truth_table:
        return 1.0 if truth_table[(w, x)] == q else 0.0
    else:
        raise RuntimeError
    
class LogicFunction(Enum):
    WIRE = 'line'
    INVERTER = 'inverter'
    MAJORITY = 'majority'
    MEMORY_CELL = 'memory-cell'

def analyze_simulation_file(file_path: str, comparison_mode: LogicFunction, clock_delays: list[str]) -> float:
    table = _run_analysis(file_path, clock_delays)

    if comparison_mode == LogicFunction.WIRE:
        cmp_func = _cmp_var_line
    elif comparison_mode == LogicFunction.INVERTER:
        cmp_func = _cmp_inverter
    elif comparison_mode == LogicFunction.MAJORITY:
        cmp_func = _cmp_majority
    elif comparison_mode == LogicFunction.MEMORY_CELL:
        cmp_func = _cmp_memory_cell
    else:
        raise RuntimeError(f'Unknown comparison mode: {comparison_mode}')
    
    accuracy = _calculate_table_accuracy(table, cmp_func)

    return accuracy

def get_simulation_file_config(file_path: str) -> tuple[float, float]:
    base_name = os.path.splitext(os.path.basename(file_path))[0]
    parts = base_name.split('_')
    x = float(parts[-2])
    y = float(parts[-1])
    return (x, y)

def analyze_simulation_files(file_paths: list[str], comparison_mode: LogicFunction, clock_delays: list[str]) -> list[tuple[float, float, float]]:
    results = []
    for file_path in file_paths:
        accuracy = analyze_simulation_file(file_path, comparison_mode, clock_delays)
        (x, y) = get_simulation_file_config(file_path)
        results.append((x, y, accuracy))
    return results

def write_analysis_to_csv(results: list[tuple[float, float, float]], output_path: str) -> None:
    df = pd.DataFrame(results, columns=['x_coord', 'y_coord', 'accuracy'])
    df.to_csv(output_path, index=False)

if __name__ == "__main__":

    if len(sys.argv) < 3:
        print('Usage: python analyze_truth.py <filename> <line|not|majority> [<cell_delay>,...]')
        sys.exit(1)

    input_dir = sys.argv[1]
    logic_function = LogicFunction(sys.argv[2])
    delays = sys.argv[3:]

    simulation_files = []
    print(f'Searching for files in: {input_dir}...')
    for file in os.scandir(input_dir):
        if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcs'):
            continue
        simulation_files.append(file.path)

    print(f'Analyzing {len(simulation_files)} files.')

    result = analyze_simulation_files(simulation_files, logic_function, delays)

    output = f'{input_dir}/truth_analysis.csv'
    write_analysis_to_csv(result, output)

    print(f'Saved truth analysis to: {output}')