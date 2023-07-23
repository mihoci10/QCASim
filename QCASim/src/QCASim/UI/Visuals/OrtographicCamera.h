#pragma once

#include <Cherry/Camera.hpp>

namespace QCAS {

    class OrtographicCamera : public Cherry::Camera
    {
    public:
        OrtographicCamera(float left, float right, float bottom, float top);

        glm::vec3 GetPosition() const { return m_Position; };
        glm::vec3 GetRotation() const { return m_Rotation; };
        float GetZoom() const { return m_Zoom;  };
        const glm::mat4& GetView() const { return m_View; }
        const glm::mat4& GetViewProjection() const { return m_CachedViewProject; }

        void SetPosition(glm::vec3 position);
        void SetRotation(glm::vec3 rotation);
        void SetZoom(float zoom);
        void SetView(float left, float right, float bottom, float top);

    private:
        void RecalcCachedData();

        std::array<float, 4> m_ProjectionParams;
        glm::vec3 m_Position {0.0f}, m_Rotation{ 0.0f };
        float m_Zoom = 1.0f;
        glm::mat4 m_View;
        glm::mat4 m_CachedViewProject;

    };

}