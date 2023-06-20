#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class StatsFrame : public BaseFrame {
    public:
        StatsFrame(const AppContext& appContext) : BaseFrame(appContext) {};
        virtual void Render() override;

    };

}