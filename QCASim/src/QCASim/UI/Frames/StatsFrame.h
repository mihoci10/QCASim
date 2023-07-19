#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class StatsFrame : public BaseFrame {
    public:
        StatsFrame(const QCASim& app) : BaseFrame(app) {};

        virtual void Render() override;

    };

}