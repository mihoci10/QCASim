import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from multiprocessing import cpu_count

QCA_SIM = '../target/release/qca-sim'


def run_simulation(filename: str, output: str):
    try:
        result = subprocess.run([QCA_SIM, 'sim', filename, '-o', output],
                                capture_output=True, text=True, check=True)
        return f"✓ Completed: {os.path.basename(filename)}"
    except subprocess.CalledProcessError as e:
        return f"✗ Failed: {os.path.basename(filename)} - {e.stderr.strip()}"
    except Exception as e:
        return f"✗ Error: {os.path.basename(filename)} - {str(e)}"


def main():
    if len(sys.argv) != 2:
        print('Usage: python run_sim.py <input_directory>')
        sys.exit(1)

    input_dir = sys.argv[1]
    qcd_files = []
    print(f'Searching for files in: {input_dir}...')

    for file in os.scandir(input_dir):
        if not file.is_file() or not (os.path.splitext(file.path)[-1] == '.qcd'):
            continue
        base_name = os.path.splitext(os.path.basename(file.path))[0]
        qcd_files.append((file.path, f'{input_dir}/{base_name}.qcs'))
        print(f'Found file: {base_name}')

    if not qcd_files:
        print("No .qcd files found!")
        return

    print(f'\nRunning {len(qcd_files)} simulations on {cpu_count()} CPU cores...')

    # Run simulations in parallel
    with ThreadPoolExecutor(max_workers=cpu_count()) as executor:
        # Submit all jobs
        future_to_file = {
            executor.submit(run_simulation, filename, output): filename
            for filename, output in qcd_files
        }

        # Process completed jobs
        completed = 0
        for future in as_completed(future_to_file):
            filename = future_to_file[future]
            result = future.result()
            completed += 1
            print(f"[{completed}/{len(qcd_files)}] {result}")

    print(f"\nAll {len(qcd_files)} simulations completed!")


if __name__ == "__main__":
    main()