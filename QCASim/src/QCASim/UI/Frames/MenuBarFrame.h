#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class MenuBarFrame : public BaseFrame {
    public:
        MenuBarFrame(const QCASim& app) : BaseFrame(app) {};
        virtual void Render() override;
    };

}