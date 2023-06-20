#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class MenuBarFrame : public BaseFrame {
    public:
        MenuBarFrame(const AppContext& appContext) : BaseFrame(appContext) {};
        virtual void Render() override;

    };

}