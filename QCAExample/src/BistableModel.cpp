#include "BistableModel.h"

#define QCHARGE_SQUAR_OVER_FOUR 6.417423538e-39
#define FOUR_PI_EPSILON 1.112650056e-10

void BistableModel::Configure(const QCAS::SimulationModelConfig& config)
{
    m_Config = config;
}

void BistableModel::Initiate(std::vector<QCAS::Cell>& cells)
{
    m_Cells.clear();
    
    for (uint32_t i = 0; i < cells.size(); i++)
        m_Cells.push_back(BCell(cells[i]));

    for (uint32_t i = 0; i < cells.size(); i++)
        for (uint32_t j = 0; j < cells.size(); j++)
            if (i != j)
            {
                m_Cells[i].NeighbourCellIndecies.push_back(j);
                m_Cells[i].NeighbourKinkEnergy.push_back(
                    DetermineKinkEnergy(m_Cells[i], m_Cells[j]));
            }
}

void BistableModel::PreCalculate(const std::array<double, 4>& clockStates)
{
    m_ClockStates = clockStates;
}

bool BistableModel::Calculate(uint32_t cellIndex)
{
    BCell& bCell = m_Cells[cellIndex];
    if (bCell.Cell.CellType == QCAS::CellType::Fixed)
        return true;
    double oldPolarization = bCell.Cell.Polarization;
    double polarMath = 0;
    for (size_t i = 0; i < bCell.NeighbourCellIndecies.size(); i++)
        polarMath += (bCell.NeighbourKinkEnergy[i] * m_Cells[bCell.NeighbourCellIndecies[i]].Cell.Polarization);

    polarMath /= (2.0 * 9.800000e-022); //m_ClockStates[bCell.Cell.GetClockIndex()]);

    double newPolarization;
    if (polarMath > 1000)
        newPolarization = 1;
    else if (polarMath < -1000)
        newPolarization = -1;
    else if (std::fabs(polarMath) < 0.001)
        newPolarization = polarMath;
    else
        newPolarization = polarMath / std::sqrt(1 + polarMath * polarMath);

    bCell.Cell.Polarization = newPolarization;
    
    return (std::fabs(newPolarization - oldPolarization) <= m_Config.Tolerance);
}

double BistableModel::DetermineKinkEnergy(BCell cell, BCell otherCell)
{
    int k;
    int j;

    double distance = 0;
    double Constant = 1 / (FOUR_PI_EPSILON * 12.900000);

    static double same_polarization[4][4] =
    { {  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR },
     { -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR },
     {  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR },
     { -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR } };

    static double diff_polarization[4][4] =
    { { -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR },
     {  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR },
     { -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR },
     {  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR,  QCHARGE_SQUAR_OVER_FOUR, -QCHARGE_SQUAR_OVER_FOUR } };

    double EnergyDiff = 0;
    double EnergySame = 0;

    for (k = 0; k < cell.Cell.QDots.size() ; k++)
        for (j = 0; j < otherCell.Cell.QDots.size(); j++)
        {
            double x = std::fabs(cell.Cell.QDots[k].Position.x - otherCell.Cell.QDots[j].Position.x);
            double y = std::fabs(cell.Cell.QDots[k].Position.y - otherCell.Cell.QDots[j].Position.y);

            distance = 1e-9 * std::sqrt(x * x + y * y);
            EnergyDiff += diff_polarization[k][j] / distance;
            EnergySame += same_polarization[k][j] / distance;
        }

    return Constant * (EnergyDiff - EnergySame);
}
