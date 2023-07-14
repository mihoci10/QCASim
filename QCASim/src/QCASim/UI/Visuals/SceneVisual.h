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
        SceneVisual(const QCAS::AppContext& appContext);

        virtual void Render() override;
        virtual uint32_t GetTextureID() override;

        virtual void SetSize(uint32_t width, uint32_t height) override;

    private:

        std::shared_ptr<Cherry::VertexBuffer> m_Buffer;
        std::shared_ptr<Cherry::Shader> m_Shader;
        std::shared_ptr<Cherry::Framebuffer> m_Framebuffer;
        std::shared_ptr<OrtographicCamera> m_Camera;
    };

}