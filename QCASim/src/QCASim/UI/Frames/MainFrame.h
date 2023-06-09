#pragma once

#include <QCASim/UI/Frames/IFrame.hpp>
#include <QCASim/UI/Frames/MenuBarFrame.h>

namespace QCAS{

    class MainFrame : public IFrame {
    public:
        MainFrame();
        virtual void Render() override;

    private:
        std::unique_ptr<MenuBarFrame> m_MenuBarFrame;

    };

}