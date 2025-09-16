import numpy as np
import sys
from load_sim import load_sim_file
import os

def get_simulation_file_config(file_path: str) -> tuple[float, float]:
    base_name = os.path.splitext(os.path.basename(file_path))[0]
    parts = base_name.split('_')
    x = float(parts[-2])
    y = float(parts[-1])
    return (x, y)

def get_simulation_time(file_path: str) -> float:
    (_, metadata, _, _) = load_sim_file(file_path)

    sim_time = metadata['duration']
    time_ms = sim_time[0] + sim_time[1] / 1e9
    return time_ms

def get_simulation_times(file_paths: list[str]) -> list[float]:
    results = []
    for file_path in file_paths:
        sim_time = get_simulation_time(file_path)
        results.append(sim_time)
    return results

def analyze_simulation_times(file_paths: list[str]):
    times = get_simulation_times(file_paths)
    avg_time = np.mean(times)
    min_time = np.min(times)
    max_time = np.max(times)
    std_time = np.std(times)
    return (avg_time, min_time, max_time, std_time)

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("First arguments need to be folders with *.qcs files!")
        sys.exit(1)

    input_dirs = sys.argv[1:]
    simulation_files = []
    for input_dir in input_dirs:
        print(f'Searching for files in: {input_dir}...')
        for file in os.scandir(input_dir):
            if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcs'):
                continue
            simulation_files.append(file.path)
    
    print(f'Found {len(simulation_files)} simulation files.')
    (avg_time, min_time, max_time, std_time) = analyze_simulation_times(simulation_files)
    print(f'Average simulation time: {avg_time:.2f} s')
    print(f'Minimum simulation time: {min_time:.2f} s')
    print(f'Maximum simulation time: {max_time:.2f} s')
    print(f'Standard deviation: {std_time:.2f} s')