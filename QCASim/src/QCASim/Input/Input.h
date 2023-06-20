#pragma once

#include <QCASim/AppContext.hpp>
#include <SDL.h>

namespace QCAS{

    class Input {
    public:
        Input(const AppContext& appContext);
        ~Input();

        bool GetKeyDown(SDL_KeyCode keyCode);
        bool GetMouseKeyDown(Uint8 keyCode);

    private:
        void OnEvent(SDL_Event* ev);

        std::function<void()> m_OnQuit;

        std::unordered_map<SDL_Keycode, bool> m_KeyStatus;
        std::unordered_map<Uint8, bool> m_MouseStatus;

        friend class QCASim;
    };

}