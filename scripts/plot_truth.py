import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import sys
from collections import defaultdict
from scipy.interpolate import griddata

if len(sys.argv) < 2:
    print("Usage: python plot_truth.py <csv_file1> [<csv_file2> ...]")
    print("At least one CSV file is required.")
    sys.exit(1)

# Dictionary to store coordinates and their accuracies
coord_accuracies = defaultdict(list)
all_coords = set()
file_count = len(sys.argv) - 1

# Process each input file
for i, filename in enumerate(sys.argv[1:], 1):
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

fig = plt.figure(figsize=(10, 5))

# Create a grid for interpolation
x_min, x_max = x_coords.min(), x_coords.max()
y_min, y_max = y_coords.min(), y_coords.max()

# Create a finer grid for smooth contours
grid_x, grid_y = np.meshgrid(
    np.linspace(x_min, x_max, min(100, len(x_coords))),
    np.linspace(y_min, y_max, min(100, len(y_coords)))
)

# Interpolate the accuracy values onto the grid
grid_accuracy = griddata(
    (x_coords, y_coords), accuracies, (grid_x, grid_y),
    method='cubic', fill_value=np.nan
)

# Clamp interpolated values to valid accuracy range [0, 1]
grid_accuracy = np.clip(grid_accuracy, 0.0, 1.0)

# Create the contour plot
ax1 = plt.subplot(1, 1, 1)
contour_filled = ax1.contourf(grid_x, grid_y, grid_accuracy, levels=20,
                              cmap='viridis', alpha=0.8)
# contour_lines = ax1.contour(grid_x, grid_y, grid_accuracy, levels=20,
#                             colors='black', alpha=0.4, linewidths=0.5)
#
# # Add contour labels
# ax1.clabel(contour_lines, inline=True, fontsize=12, fmt='%.2f')

ax1.set_xlabel('Intercell distance ($nm$)', fontsize=12)
ax1.set_ylabel('Quantum dot radius ($nm$)', fontsize=12)
ax1.grid(True, alpha=0.3)

# Add colorbar
cbar1 = plt.colorbar(contour_filled, ax=ax1, shrink=0.8)
cbar1.set_label('Accuracy', fontsize=12)

plt.tight_layout()
plt.savefig('TruthAnalysis.pdf')
plt.show()
