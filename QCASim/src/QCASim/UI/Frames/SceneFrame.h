#pragma once

#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class SceneFrame : public BaseFrame {
    public:
        SceneFrame(const AppContext& appContext) : BaseFrame(appContext) {};
        virtual void Render() override;

    };

}