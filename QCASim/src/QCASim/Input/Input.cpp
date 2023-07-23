#include "Input.h"

#include <QCASim/QCASim.h>

namespace QCAS{

    Input::Input(const QCASim& app) : QCASimComponent(app)
    {
    }

    Input::~Input()
    {
    }

    bool Input::GetKeyDown(ImGuiKey key) const
    {
        return ImGui::IsMouseClicked(key);
    }

    bool Input::GetMouseKeyDown(ImGuiMouseButton mouse) const
    {
        return ImGui::IsMouseDown(mouse);
    }

    ImVec2 Input::GetMousePositionDelta() const
    {
        return ImGui::GetIO().MouseDelta;
    }

    float Input::GetMouseWheelDelta() const
    {
        return ImGui::GetIO().MouseWheel;
    }

    void Input::OnEvent(SDL_Event* ev)
    {
        auto io = ImGui::GetIO();
        m_App.GetGraphics().GetImGuiApi().OnEvent(ev);
        
        switch (ev->type) {
            case SDL_EVENT_QUIT:
                m_OnQuit();
                break;
        }
    }

}