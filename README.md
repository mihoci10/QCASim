# QCASim

A Quantum Cellular Automata (QCA) simulation framework for academic research, providing tools to model and analyze quantum dot cellular automata circuits.

## Features

- **Simulation engine**: Supports built-in bistable and ICHA model with option to use custom models as well.
- **File Formats**: Defines and uses `.qcd` (QCA Design) and `.qcs` (QCA Simulation) file formats
- **Truth Table Analysis**: Generate and analyze logic truth tables from simulation results
- **CLI Interface**: Command-line tools for simulation and analysis
- **Example Designs**: Includes wire, inverter, majority gate, and memory cell designs
- **Analysis**: Interactive analysis scripts for simulation data

## Installation

```bash
git clone https://github.com/mihoci10/QCASim.git
cd QCASim
cargo build --release
```

## Usage

### Run Simulation

```bash
qca-sim sim examples/line.qcd
```

### Analysis

Use the Jupyter notebook `scripts/analysis.ipynb` for interactive analysis of simulation results and visualization.

## File Formats

- **`.qcd`**: QCA design files containing circuit layout, cell positions, and simulation parameters
- **`.qcs`**: QCA simulation files containing simulation results and metadata

## Project Structure

- `qca-core/`: Core simulation engine and data structures
- `qca-sim/`: CLI application for running and analyzing simulations
- `examples/`: Example QCA designs
- `scripts/`: Python analysis tools and batch processing utilities

## Status

This project is stable and under active development for academic research applications.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
