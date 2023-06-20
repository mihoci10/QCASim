#include "QCASim.h"

#include <QCASim/UI/Graphics.h>
#include <QCASim/Input/Input.h>
#include <QCASim/Data/MachineStats.h>

namespace QCAS
{
    void QCASim::Startup()
    {
        m_ShouldRestart = false;

        m_AppContext = std::make_unique<AppContext>();

        m_AppContext->m_Graphics = std::make_unique<Graphics>(*m_AppContext.get(), std::make_shared<Cherry::RendererSettings>(Cherry::RendererPlatform::OpenGL, true));
        
        m_AppContext->m_Input = std::make_unique<Input>(*m_AppContext.get());
        m_AppContext->GetInput().m_OnQuit = [&]() { m_Running = false; };

        m_AppContext->m_MachineStats = std::make_unique<MachineStats>(*m_AppContext.get());

        m_AppContext->m_Initialized = true;
    }

    void QCASim::Run()
    {
        m_Running = true;

        SDL_Event ev;

        while (m_Running) {
            m_AppContext->GetMachineStats().StartFrame();

            while (SDL_PollEvent(&ev) != 0) { 
                m_AppContext->GetGraphics().GetImGuiApi().OnEvent(&ev);
                m_AppContext->GetInput().OnEvent(&ev);
            };

            m_AppContext->GetGraphics().BeginFrame();
            m_AppContext->GetGraphics().RenderFrame();
            m_AppContext->GetGraphics().EndFrame();

            m_AppContext->GetMachineStats().EndFrame();
        }
    }

    void QCASim::Shutdown()
    {
        m_AppContext.reset();
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