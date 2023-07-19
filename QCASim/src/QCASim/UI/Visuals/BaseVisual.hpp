#pragma once

#include <QCASim/QCASimComponent.hpp>
#include <Cherry/RendererSettings.hpp>
#include <glm/glm.hpp>

namespace QCAS {

    struct VisualInitContext {
        const QCASim& app;
        const Cherry::RendererSettings& rendererSettings;
    };

    class BaseVisual : public QCASimComponent
    {
    public:
        virtual ~BaseVisual() {};

        virtual void Render() = 0;

        virtual uint32_t GetTextureID() const = 0;

        virtual void SetSize(uint32_t width, uint32_t height) { m_Width = width; m_Height = height; };
        inline glm::vec2 GetSize() const { return { m_Width, m_Height }; };

    protected:
        BaseVisual(const VisualInitContext& context) : QCASimComponent(context.app) {};

        uint32_t m_Width = 0, m_Height = 0;
    };

}