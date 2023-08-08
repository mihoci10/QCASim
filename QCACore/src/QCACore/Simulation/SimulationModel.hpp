#pragma once

#include <QCACore/Simulation/Cell.hpp>
#include <QCACore/Simulation/SimulationModelConfig.hpp>

namespace QCAC::Sim {

	class SimulationModel {
	public:
		virtual ~SimulationModel() {};

		virtual void Configure(const SimulationModelConfig& config) = 0;
		virtual void Initiate(std::vector<Cell>& cells) = 0;
		virtual void PreCalculate(const std::array<double, 4>& clockStates) = 0;
		virtual bool Calculate(uint32_t cellIndex) = 0;
	};

}