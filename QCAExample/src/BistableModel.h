#pragma once

#include <QCAS/Cell.hpp>
#include <QCAS/SimulationModel.hpp>

struct BCell {
	BCell(QCAS::Cell& cell) : Cell(cell) {};
	QCAS::Cell& Cell;
	std::vector<uint32_t> NeighbourCellIndecies;
	std::vector<double> NeighbourKinkEnergy;
};

class BistableModel : public QCAS::SimulationModel {
public:
	virtual void Configure(const QCAS::SimulationModelConfig& config) override;
	virtual void Initiate(std::vector<QCAS::Cell>& cells) override;
	virtual void PreCalculate(const std::array<double, 4>& clockStates) override;
	virtual bool Calculate(uint32_t cellIndex) override;

private:
	double DetermineKinkEnergy(BCell cell, BCell otherCell);

	QCAS::SimulationModelConfig m_Config;
	std::vector<BCell> m_Cells;
	std::array<double, 4> m_ClockStates;
};