import sys
from collections import defaultdict

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy.interpolate import griddata

def _merge_truth_analysis(truth_analysis_filenames: list[str]) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    # Dictionary to store coordinates and their accuracies
    coord_accuracies = defaultdict(list)
    all_coords = set()
    file_count = len(truth_analysis_filenames)

    # Process each input file
    for filename in truth_analysis_filenames:
        input_data = pd.read_csv(filename)

        # Extract coordinates and accuracies
        for _, row in input_data.iterrows():
            coord = (row['x_coord'], row['y_coord'])
            accuracy = row['accuracy']
            coord_accuracies[coord].append(accuracy)
            all_coords.add(coord)

    # Check for missing coordinates and multiply accuracies
    final_x_coords = []
    final_y_coords = []
    final_accuracies = []

    for coord in all_coords:
        x, y = coord
        accuracies_at_coord = coord_accuracies[coord]

        # Check if this coordinate is missing in any file
        if len(accuracies_at_coord) < file_count:
            missing_count = file_count - len(accuracies_at_coord)
            print(f"Warning: Coordinate ({x}, {y}) is missing in {missing_count} file(s)")

        # Multiply accuracies for this coordinate
        multiplied_accuracy = 1.0
        for acc in accuracies_at_coord:
            multiplied_accuracy *= acc

        final_x_coords.append(x)
        final_y_coords.append(y)
        final_accuracies.append(multiplied_accuracy)

    # Convert to numpy arrays
    x_coords = np.array(final_x_coords)
    y_coords = np.array(final_y_coords)
    accuracies = np.array(final_accuracies)

    return (x_coords, y_coords, accuracies)

def visualize_truth_analysis(truth_analysis_filenames: list[str], output_filename: str|None):
    (x_coords, y_coords, accuracies) = _merge_truth_analysis(truth_analysis_filenames)

    fig = plt.figure(figsize=(10, 5))

    x_min, x_max = x_coords.min(), x_coords.max()
    y_min, y_max = y_coords.min(), y_coords.max()

    grid_x, grid_y = np.meshgrid(
        np.linspace(x_min, x_max, min(100, len(x_coords))),
        np.linspace(y_min, y_max, min(100, len(y_coords)))
    )

    grid_accuracy = griddata(
        (x_coords, y_coords), accuracies, (grid_x, grid_y),
        method='cubic', fill_value=np.nan
    )

    grid_accuracy = np.clip(grid_accuracy, 0.0, 1.0)

    ax1 = plt.subplot(1, 1, 1)
    contour_filled = ax1.contourf(grid_x, grid_y, grid_accuracy,
                                cmap='viridis', alpha=1.0)

    ax1.set_xlabel('Medcelična razdalja ($nm$)', fontsize=12)
    ax1.set_ylabel('Radij razporeditve kvantnih pik ($nm$)', fontsize=12)
    ax1.grid(True, alpha=0.3)

    cbar1 = plt.colorbar(contour_filled, ax=ax1, shrink=1.0)
    cbar1.set_label('Pravilnost', fontsize=12)

    plt.tight_layout()
    if output_filename is not None:
        plt.savefig(output_filename)
    plt.show()

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python plot_truth.py <csv_file1> [<csv_file2> ...]")
        print("At least one CSV file is required.")
        sys.exit(1)

    visualize_truth_analysis(sys.argv[1:], None)