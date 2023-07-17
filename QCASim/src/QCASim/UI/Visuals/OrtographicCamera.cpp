#include "OrtographicCamera.h"

#include <glm/gtc/matrix_transform.hpp>

namespace QCAS{

    OrtographicCamera::OrtographicCamera(float left, float right, float bottom, float top)
        : Camera()
    {
        SetView(left, right, bottom, top);
    }

    void OrtographicCamera::SetPosition(glm::vec3 position)
    {
        m_Position = position;
        RecalcCachedData();
    }

    void OrtographicCamera::SetRotation(glm::vec3 rotation)
    {
        m_Rotation = rotation;
        RecalcCachedData();
    }

    void OrtographicCamera::SetZoom(float zoom)
    {
        m_Zoom = zoom;
        RecalcCachedData();
    }

    void OrtographicCamera::SetView(float left, float right, float bottom, float top)
    {
        m_ProjectionParams = { left, right, bottom, top };
        RecalcCachedData();
    }

    void OrtographicCamera::RecalcCachedData()
    {
        glm::mat4 transform = glm::translate(glm::mat4(1.0f), m_Position)
            * glm::rotate(glm::mat4(1.0f), glm::radians(m_Rotation.x), glm::vec3(1, 0, 0))
                * glm::rotate(glm::mat4(1.0f), glm::radians(m_Rotation.y), glm::vec3(0, 1, 0))
                    * glm::rotate(glm::mat4(1.0f), glm::radians(m_Rotation.z), glm::vec3(0, 0, 1));

        m_View = glm::inverse(transform);
        m_Projection = glm::ortho(m_ProjectionParams[0] / m_Zoom, 
            m_ProjectionParams[1] / m_Zoom, 
            m_ProjectionParams[2] / m_Zoom, 
            m_ProjectionParams[3] / m_Zoom);
        m_CachedViewProject = m_Projection * m_View;
    }

}