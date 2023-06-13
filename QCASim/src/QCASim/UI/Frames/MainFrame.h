#pragma once

#include <QCASim/UI/Frames/IFrame.hpp>
#include <QCASim/UI/Frames/MenuBarFrame.h>
#include <QCASim/UI/Frames/SceneFrame.h>
#include <QCASim/UI/Frames/StatsFrame.h>

namespace QCAS{

    class MainFrame : public IFrame {
    public:
        MainFrame();
        virtual void Render() override;

    private:
        std::unique_ptr<MenuBarFrame> m_MenuBarFrame;
        std::unique_ptr<SceneFrame> m_SceneFrame;
        std::unique_ptr<StatsFrame> m_StatsFrame;

    };

}