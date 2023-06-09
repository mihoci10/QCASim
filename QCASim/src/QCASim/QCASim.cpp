#include "QCASim.h"

#include <Cherry/RendererSettings.hpp>
#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS
{
    void QCASim::Startup()
    {
        m_ShouldRestart = false;

        Graphics::Initialize(std::make_shared<Cherry::RendererSettings>(Cherry::RendererPlatform::OpenGL, true));
        
        Input::Initialize();
        Input::GetInstance().m_OnQuit = [&]() { m_Running = false; };
    }

    void QCASim::Run()
    {
        m_Running = true;

        SDL_Event ev;

        while (m_Running) {

            while (SDL_PollEvent(&ev) != 0) { 
                Graphics::GetInstance().GetImGuiApi().OnEvent(&ev);
                Input::GetInstance().OnEvent(&ev); 
            };

            Graphics::GetInstance().BeginFrame();
            Graphics::GetInstance().RenderFrame();
            Graphics::GetInstance().EndFrame();
        }
    }

    void QCASim::Shutdown()
    {
        Input::Deinitialize();
        Graphics::Deinitialize();
    }

}


int main(int argc, char** argv) {

    QCAS::QCASim app;

    do {
        app.Startup();

        app.Run();

        app.Shutdown();
    } while (app.ShouldRestart());

     std::exit(0);
}