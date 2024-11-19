# QCASim

QCASim is a Quantum Cellular Dot Automata Simulator designed to model and simulate the behavior of quantum cellular automata structures. This project aims to provide researchers and enthusiasts with a tool to explore and analyze quantum computational models.

## Features

- **Quantum State Simulation**: Simulate the evolution of quantum states in a cellular automata framework.
- **Custom Simulation Dynamics**: Define, implement and use custom simulation dynamics.
- **Visualization**: Visualize the state of the quantum cellular automata over time.

## Installation

To install QCASim, clone the repository and install the required dependencies:

```bash
git clone https://github.com/yourusername/QCASim.git
cd QCASim
cargo build
```

## Usage

### Running simulations

To run a simulation, use the following command:

```bash
cargo run --bin qca-sim -- path/to/design.json
```

### Analyzing results

To analyze the simulation output use the following command:

```bash
python utils/data-analyzer.py path/to/output.bin
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
