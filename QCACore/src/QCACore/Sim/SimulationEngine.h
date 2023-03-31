#pragma once

#include <QCACore/Sim/Cell.hpp>
#include <QCACore/Sim/SimulationModel.hpp>
#include <QCACore/Sim/SimulationSettings.hpp>
#include <QCACore/Sim/SimulationModelConfig.hpp>

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