#pragma once

#include <QCASim/UI/Visuals/BaseVisual.hpp>
#include <QCASim/UI/Visuals/OrtographicCamera.h>

#include <Cherry/Buffer.h>
#include <Cherry/Framebuffer.h>
#include <Cherry/Shader.h>

namespace QCAS {

    class SceneVisual : public BaseVisual
    {
    public:
        SceneVisual(const QCASim& app);

        virtual void Render() override;

        virtual uint32_t GetTextureID() const override;

        virtual void SetSize(uint32_t width, uint32_t height) override;

    private:
        std::unique_ptr<Cherry::VertexBuffer> m_Buffer;
        std::unique_ptr<Cherry::Shader> m_Shader;
        std::unique_ptr<Cherry::Framebuffer> m_Framebuffer;
        std::unique_ptr<OrtographicCamera> m_Camera;
    };

}