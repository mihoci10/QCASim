#pragma once

#include <QCASim/UI/Visuals/BaseVisual.hpp>
#include <QCASim/UI/Visuals/OrtographicCamera.h>

#include <Cherry/BufferBatch.h>
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
        void InitWorldGrid();
        void InitCells();

        std::unique_ptr<Cherry::BufferBatch> m_CellsBufferBatch;
        std::unique_ptr<Cherry::VertexBuffer> m_GridBuffer;
        std::unique_ptr<Cherry::Shader> m_GridShader, m_CellShader;
        std::unique_ptr<Cherry::Framebuffer> m_Framebuffer;
        std::unique_ptr<OrtographicCamera> m_Camera;

        struct CellData {
            float GlobalPos[3];
            float VertexPos[3];
            float Color[4];
        };
    };

}