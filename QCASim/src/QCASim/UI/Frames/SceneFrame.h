#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>
#include <QCASim/UI/Visuals/SceneVisual.h>

namespace QCAS{

    class SceneFrame : public BaseFrame {
    public:
        SceneFrame(const AppContext& appContext);
        virtual void Render() override;

    private:
        std::unique_ptr<SceneVisual> m_Visual;
    };

}