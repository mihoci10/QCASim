#include "QCASim.h"

namespace QCAS
{
    void QCASim::Startup()
    {
        m_ShouldRestart = false;

        m_Graphics = std::make_unique<Graphics>(*this, Cherry::RendererSettings(Cherry::RendererPlatform::OpenGL, true));

        m_MainFrame = std::make_unique<MainFrame>(*this);
        
        m_Input = std::make_unique<Input>(*this);
        m_Input->m_OnQuit = [&]() { m_Running = false; };

        m_MachineStats = std::make_unique<MachineStats>(*this);
    }

    void QCASim::Run()
    {
        m_Running = true;

        SDL_Event ev;

        while (m_Running) {
            m_MachineStats->BeginFrame();

            while (SDL_PollEvent(&ev) != 0) { 
                m_Graphics->GetImGuiApi().OnEvent(&ev);
                m_Input->OnEvent(&ev);
            };

            m_Graphics->BeginFrame();
            m_MainFrame->Render();
            m_Graphics->EndFrame();

            m_MachineStats->EndFrame();
        }
    }

    void QCASim::Shutdown()
    {
        m_MachineStats.reset();
        m_Input.reset();
        m_MainFrame.reset();
        m_Graphics.reset();
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