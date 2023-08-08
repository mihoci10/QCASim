#pragma once

#include <QCACore/Simulation/Cell.hpp>
#include <QCACore/Simulation/SimulationModel.hpp>
#include <QCACore/Simulation/SimulationSettings.hpp>
#include <QCACore/Simulation/SimulationModelConfig.hpp>

namespace QCAC::Sim {

	class SimulationEngine {
	public:
		SimulationEngine() = default;
		~SimulationEngine() = default;

		void SetCells(std::vector<Cell> cells);
		void SetModel(std::shared_ptr<SimulationModel> model);
		void Start();
		void Stop();

		void SetModelConfig(const SimulationModelConfig& config);

		SimulationSettings Settings = {};
	private:
		std::shared_ptr<SimulationModel> m_SimModel = nullptr;
		std::vector<Cell> m_Cells = {};
		Cell* cells;
	};

}