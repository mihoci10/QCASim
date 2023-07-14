#include "OrtographicCamera.h"

#include <glm/gtc/matrix_transform.hpp>

namespace QCAS{

    OrtographicCamera::OrtographicCamera(float left, float right, float bottom, float top)
        : Camera(glm::ortho(left, right, bottom, top))
    {
        RecalcCachedData();
    }

    void OrtographicCamera::RecalcCachedData()
    {
        glm::mat4 transform = glm::translate(glm::mat4(1.0f), m_Position)
            * glm::rotate(glm::mat4(1.0f), glm::radians(m_Rotation.x), glm::vec3(1, 0, 0))
                * glm::rotate(glm::mat4(1.0f), glm::radians(m_Rotation.y), glm::vec3(0, 1, 0))
                    * glm::rotate(glm::mat4(1.0f), glm::radians(m_Rotation.z), glm::vec3(0, 0, 1));

        m_View = glm::inverse(transform);
        m_CachedViewProject = m_Projection * m_View;
    }

}