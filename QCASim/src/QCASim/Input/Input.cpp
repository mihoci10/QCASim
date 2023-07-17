#include "Input.h"

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS{

    Input::Input(const QCASim& app) : QCASimComponent(app)
    {
    }

    Input::~Input()
    {
    }

    bool Input::GetKeyDown(SDL_KeyCode keyCode)
    {
        auto k = m_KeyStatus.find(keyCode);
        if (k == m_KeyStatus.end())
            return false;
        return k->second;
    }

    bool Input::GetMouseKeyDown(Uint8 keyCode)
    {
        auto k = m_MouseStatus.find(keyCode);
        if (k == m_MouseStatus.end())
            return false;
        return k->second;
    }

    void Input::OnEvent(SDL_Event* ev)
    {
        auto io = ImGui::GetIO();
        
        switch (ev->type) {
            case SDL_EVENT_MOUSE_BUTTON_DOWN:
            case SDL_EVENT_MOUSE_BUTTON_UP:
                if (!io.WantCaptureMouse)
                    m_MouseStatus[ev->button.button] = ev->button.state == SDL_PRESSED;
                break;

            case SDL_EVENT_KEY_DOWN:
            case SDL_EVENT_KEY_UP:
                if(!io.WantCaptureKeyboard)
                    m_KeyStatus[ev->key.keysym.sym] = ev->key.state == SDL_PRESSED;
                break;

            case SDL_EVENT_QUIT:
                m_OnQuit();
                break;
        }
    }

}