#include "SimulationEngine.h"

namespace QCAC::Sim {
	void SimulationEngine::SetCells(std::vector<Cell> cells)
	{
		m_Cells = cells;
	}
	void SimulationEngine::SetModel(std::shared_ptr<SimulationModel> model)
	{
		m_SimModel = model;
	}
	void SimulationEngine::Start()
	{
		m_SimModel->Initiate(m_Cells);

		for (uint32_t i = Settings.StartSample; i <= Settings.EndSample; i++)
		{
			m_SimModel->PreCalculate({ 0, 0, 0, 0 });

			bool stable = false;
			while (!stable) {
				stable = true;

				for (uint32_t i = 0; i < m_Cells.size(); i++) 
					stable &= m_SimModel->Calculate(i);
			}
		}
	}

	void SimulationEngine::Stop()
	{
	}

	void SimulationEngine::SetModelConfig(const SimulationModelConfig& config)
	{
		m_SimModel->Configure(config);
	}

}