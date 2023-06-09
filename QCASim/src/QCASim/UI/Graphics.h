#pragma once 

#include <SDL.h>
#include <Cherry/RendererApi.h>
#include <Cherry/GUI/ImGuiAPI.h>
#include <QCASim/UI/Frames/IFrame.hpp>

namespace QCAS{

    class Graphics {
    public:
        static void Initialize(const std::shared_ptr<Cherry::RendererSettings>& rendererSettings);
        static void Deinitialize();
        static Graphics& GetInstance();

        void BeginFrame();
        void RenderFrame();
        void EndFrame();

        Cherry::GUI::ImGuiAPI& GetImGuiApi() const { return *m_ImGuiApi.get(); };

    private:
        Graphics(const std::shared_ptr<Cherry::RendererSettings>& rendererSettings);
        ~Graphics();

        static Graphics* s_Graphics;

        std::shared_ptr<SDL_Window> m_windowHnd;
        std::unique_ptr<Cherry::RendererAPI> m_RenderApi;
        std::shared_ptr<Cherry::RendererSettings> m_RendererSettings;
        std::unique_ptr<Cherry::GUI::ImGuiAPI> m_ImGuiApi;
        std::unique_ptr<IFrame> m_Frame;
    };

}