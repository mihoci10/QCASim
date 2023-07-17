#pragma once 

#include <QCASim/QCASimComponent.hpp>
#include <SDL.h>
#include <Cherry/RendererAPI.h>
#include <Cherry/GUI/ImGuiAPI.h>
#include <QCASim/UI/FontManager.h>
#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class Graphics : public QCASimComponent {
    public:
        Graphics(const QCASim& app, const Cherry::RendererSettings& rendererSettings);
        ~Graphics();

        void BeginFrame();
        void RenderFrame();
        void EndFrame();

        Cherry::RendererAPI& GetRendererApi() const { return *m_RenderApi.get(); };
        Cherry::GUI::ImGuiAPI& GetImGuiApi() const { return *m_ImGuiApi.get(); };
        FontManager& GetFontManager() const { return *m_FontManager.get(); };

    private:
        void SetupSDL();
        void SetupImGui();

        std::shared_ptr<SDL_Window> m_windowHnd;
        std::shared_ptr<Cherry::RendererAPI> m_RenderApi;
        Cherry::RendererSettings m_RendererSettings;
        std::unique_ptr<Cherry::GUI::ImGuiAPI> m_ImGuiApi;
        std::unique_ptr<FontManager> m_FontManager;
        std::unique_ptr<BaseFrame> m_Frame;
    };

}