#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class StatsFrame : public BaseFrame {
    public:
        StatsFrame(const FrameInitContext& context) : BaseFrame(context) {};
        virtual void Render() override;

    };

}