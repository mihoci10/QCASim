import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from multiprocessing import cpu_count

QCA_SIM = '../target/release/qca-sim'


def run_simulation(filename: str):
    try:
        result = subprocess.run([QCA_SIM, 'sim', filename],
                                capture_output=True, text=True, check=True)
        return f"✓ Completed: {os.path.basename(filename)}"
    except subprocess.CalledProcessError as e:
        return f"✗ Failed: {os.path.basename(filename)} - {e.stderr.strip()}"
    except Exception as e:
        return f"✗ Error: {os.path.basename(filename)} - {str(e)}"

def run_simulations(file_list: list[str]) -> None:
    # Run simulations in parallel
    with ThreadPoolExecutor(max_workers=cpu_count()) as executor:
        # Submit all jobs
        future_to_file = {
            executor.submit(run_simulation, filename): filename
            for filename in file_list
        }

        # Process completed jobs
        completed = 0
        for future in as_completed(future_to_file):
            filename = future_to_file[future]
            result = future.result()
            completed += 1
            print(f"[{completed}/{len(file_list)}] {result}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print('Usage: python run_sim.py <input_directory,...>')
        sys.exit(1) 

    input_dirs = sys.argv[1:]
    qcd_files = []
    print(f'Searching for files in: {input_dirs}...')

    for input_dir in input_dirs:
        if not os.path.isdir(input_dir):
            print(f'Error: {input_dir} is not a valid directory!')
            continue

        for file in os.scandir(input_dir):
            if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcd'):
                continue
            base_name = os.path.splitext(os.path.basename(file.path))[0]
            qcd_files.append(file.path)

    if not qcd_files:
        print("No .qcd files found!")
        sys.exit(0)

    print(f'\nRunning {len(qcd_files)} simulations on {cpu_count()} CPU cores...')

    run_simulations(qcd_files)

    print(f"\nAll {len(qcd_files)} simulations completed!")