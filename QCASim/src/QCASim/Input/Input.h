#pragma once

#include <QCASim/QCASimComponent.hpp>
#include <SDL.h>
#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS{

    class Input : public QCASimComponent {
    public:
        Input(const QCASim& app);
        ~Input();

        bool GetKeyDown(ImGuiKey key) const;
        bool GetMouseKeyDown(ImGuiMouseButton mouse) const;
        ImVec2 GetMousePositionDelta() const;
        float GetMouseWheelDelta() const;

    private:
        void OnEvent(SDL_Event* ev);

        std::function<void()> m_OnQuit;


        friend class QCASim;
    };

}