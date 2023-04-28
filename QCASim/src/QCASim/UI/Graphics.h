#pragma once 

#include <SDL.h>
#include <Cherry/Renderer.h>

namespace QCAS::UI{

    class Graphics{
    public:

        void Init();
        void Deinit();

        void SetRendererSettings(const std::shared_ptr<Cherry::RendererSettings>& rendererSettings) { m_RendererSettings = rendererSettings; }

    private:
        std::shared_ptr<SDL_Window> m_windowHnd;
        std::shared_ptr<Cherry::RendererSettings> m_RendererSettings;

    }

}