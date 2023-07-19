#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class MenuBarFrame : public BaseFrame {
    public:
        MenuBarFrame(const FrameInitContext& context) : BaseFrame(context) {};
        virtual void Render() override;
    };

}