import sys

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

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

# 1. Scatter plot with color-coded accuracy
ax1 = plt.subplot(2, 3, 1)
scatter = ax1.scatter(x_coords, y_coords, c=accuracies, s=120, alpha=0.8,
                      cmap='viridis', edgecolors='black', linewidth=0.5)
ax1.set_xlabel('X Coordinate', fontsize=12)
ax1.set_ylabel('Y Coordinate', fontsize=12)
ax1.set_title('Accuracy by Position (Scatter Plot)', fontsize=14, fontweight='bold')
ax1.grid(True, alpha=0.3)
cbar1 = plt.colorbar(scatter, ax=ax1, shrink=0.8)
cbar1.set_label('Accuracy', fontsize=12)

plt.show()