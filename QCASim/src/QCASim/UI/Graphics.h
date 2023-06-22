#pragma once 

#include <SDL.h>
#include <Cherry/RendererApi.h>
#include <Cherry/GUI/ImGuiAPI.h>
#include <QCASim/UI/FontManager.h>
#include <QCASim/UI/Frames/BaseFrame.hpp>

namespace QCAS{

    class Graphics {
    public:
        Graphics(const AppContext& appContext, const std::shared_ptr<Cherry::RendererSettings>& rendererSettings);
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
        std::unique_ptr<Cherry::RendererAPI> m_RenderApi;
        std::shared_ptr<Cherry::RendererSettings> m_RendererSettings;
        std::unique_ptr<Cherry::GUI::ImGuiAPI> m_ImGuiApi;
        std::unique_ptr<FontManager> m_FontManager;
        std::unique_ptr<BaseFrame> m_Frame;
    };

}