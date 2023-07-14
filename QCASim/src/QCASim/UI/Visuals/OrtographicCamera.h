#pragma once

#include <Cherry/Camera.hpp>

namespace QCAS {

    class OrtographicCamera : public Cherry::Camera
    {
    public:
        OrtographicCamera(float left, float right, float bottom, float top);

        const glm::mat4& GetView() const { return m_View; }
        const glm::mat4& GetViewProjection() const { return m_CachedViewProject; }


    private:
        void RecalcCachedData();

        glm::vec3 m_Position {0.0f}, m_Rotation{ 0.0f };

        glm::mat4 m_View;
        glm::mat4 m_CachedViewProject;

    };

}