import sys
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy.interpolate import griddata

if len(sys.argv) != 2:
    print("First argument needs to be a *.csv file!")
    sys.exit(1)

filename = sys.argv[1]
input_data = pd.read_csv(filename)

x_coords = np.array(input_data['x_coord'])
y_coords = np.array(input_data['y_coord'])
accuracies = np.array(input_data['accuracy'])

fig = plt.figure(figsize=(20, 15))
fig.suptitle('QCS Analysis Results - Example Data', fontsize=20, fontweight='bold')

# Create a grid for interpolation
x_min, x_max = x_coords.min(), x_coords.max()
y_min, y_max = y_coords.min(), y_coords.max()

# Create a finer grid for smooth contours
grid_x, grid_y = np.meshgrid(
    np.linspace(x_min, x_max, 100),
    np.linspace(y_min, y_max, 100)
)

# Interpolate the accuracy values onto the grid
grid_accuracy = griddata(
    (x_coords, y_coords), accuracies, (grid_x, grid_y),
    method='linear', fill_value=np.nan
)

# Clamp interpolated values to valid accuracy range [0, 1]
grid_accuracy = np.clip(grid_accuracy, 0.0, 1.0)

# Create the contour plot
ax1 = plt.subplot(1, 1, 1)
contour_filled = ax1.contourf(grid_x, grid_y, grid_accuracy, levels=20,
                              cmap='viridis', alpha=0.8)
contour_lines = ax1.contour(grid_x, grid_y, grid_accuracy, levels=20,
                            colors='black', alpha=0.4, linewidths=0.5)

# Add contour labels
ax1.clabel(contour_lines, inline=True, fontsize=12, fmt='%.2f')

ax1.set_xlabel('Intercell distance ($nm$)', fontsize=12)
ax1.set_ylabel('Quantum dot radius ($nm$)', fontsize=12)
ax1.grid(True, alpha=0.3)

# Add colorbar
cbar1 = plt.colorbar(contour_filled, ax=ax1, shrink=0.8)
cbar1.set_label('Accuracy', fontsize=12)

plt.tight_layout()
plt.show()