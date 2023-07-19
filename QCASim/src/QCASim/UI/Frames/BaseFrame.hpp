#pragma once

#include <QCASim/QCASimComponent.hpp>
#include <Cherry/RendererSettings.hpp>

namespace QCAS{

    struct FrameInitContext {
        const QCASim& app;
        const Cherry::RendererSettings& rendererSettings;
    };

    class BaseFrame : public QCASimComponent {
    public:
        virtual ~BaseFrame() {};

        virtual void Render() = 0;

    protected:
        BaseFrame(const FrameInitContext& context) : QCASimComponent(context.app) {};
    };

}