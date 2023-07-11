#pragma once

#include <glm/glm.hpp>
#include <QCASim/AppContext.hpp>

namespace QCAS {

    class BaseVisual
    {
    public:
        virtual ~BaseVisual() {};

        virtual void Render() = 0;
        virtual uint32_t GetTextureID() = 0;

        virtual void SetSize(uint32_t width, uint32_t height) { m_Width = width; m_Height = height; };
        inline glm::vec2 GetSize() const { return { m_Width, m_Height }; };

    protected:
        BaseVisual(const QCAS::AppContext& appContext) : m_AppContext(appContext) {};

        uint32_t m_Width = 0, m_Height = 0;
        const QCAS::AppContext& m_AppContext;
    };

}