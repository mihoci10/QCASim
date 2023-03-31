#include "BistableModel.h"

#include <QCAS/SimulationEngine.h>

void main() {

	QCAS::SimulationEngine engine;
	std::vector<QCAS::Cell> cells;

	for (size_t i = 0; i < 10; i++)
	{
		QCAS::Cell c;
		if (i == 0) {
			c.CellType = (QCAS::CellType::Fixed);
			c.Polarization = -1;
		}
		else {
			c.CellType = (QCAS::CellType::Normal);
			c.Polarization = 0;
		}
		c.Position = { i * 20.0, 0.0f };
		c.ClockIndex = (0);
		c.QDots = {
			{ {c.Position.x + 9, -9}, 2} ,
			{ {c.Position.x + 9, +9}, 2},
			{ {c.Position.x - 9, +9}, 2 },
			{ {c.Position.x - 9, -9}, 2 }
		};
		cells.push_back(c);
	}

	QCAS::Cell c;
	c.CellType = (QCAS::CellType::Normal);
	c.Polarization = 0;
	c.Position = { cells.size() * 20.0, 20.0f };
	c.ClockIndex = (0);
	c.QDots = {
		{ {c.Position.x + 9, c.Position.y - 9}, 2} ,
		{ {c.Position.x + 9, c.Position.y + 9}, 2},
		{ {c.Position.x - 9, c.Position.y + 9}, 2 },
		{ {c.Position.x - 9, c.Position.y - 9}, 2 }
	};
	cells.push_back(c);

	engine.SetModel(std::make_shared<BistableModel>());
	engine.SetCells(cells);
	engine.Start();
}