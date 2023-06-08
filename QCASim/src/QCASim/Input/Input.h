#pragma once

#include <SDL.h>

namespace QCAS{

    class Input {
    private:
        struct ImGuiInputState {
            bool wantCaptureMouse, wantCaptureKeyboard;
        };

    public:
        static void Initialize();
        static void Deinitialize();
        static Input& GetInstance();

        bool GetKeyDown(SDL_KeyCode keyCode);
        bool GetMouseKeyDown(Uint8 keyCode);

    private:
        Input();
        ~Input();

        static Input* s_Input;

        void OnEvent(SDL_Event* ev);

        std::function<void()> m_OnQuit;

        std::unordered_map<SDL_Keycode, bool> m_KeyStatus;
        std::unordered_map<Uint8, bool> m_MouseStatus;

        friend class QCASim;
    };

}