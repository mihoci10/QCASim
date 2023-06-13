#pragma once

#include <QCASim/UI/Frames/IFrame.hpp>

namespace QCAS{

    class SceneFrame : public IFrame {
    public:
        virtual void Render() override;

    };

}