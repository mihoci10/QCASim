#pragma once

namespace QCAC::Sim{

    struct Vector2 {
        double x;
        double y;
    };

    struct QDot {
        Vector2 Position = { 0.0f, 0.0f };
        double Diameter = 0.0f;
    };

    enum class CellType {
        Normal, Fixed, Input, Output
    };
    
    class Cell {
    public:
        CellType CellType = CellType::Normal;
        Vector2 Position = {0.0f, 0.0f};
        uint8_t ClockIndex = 0;
        std::vector<QDot> QDots = {};
        double Polarization = 0.0f;
    };

}