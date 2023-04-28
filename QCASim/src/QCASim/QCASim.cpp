#include "QCASim.h"

namespace QCAS
{
    void QCASim::Startup()
    {
        m_ShouldRestart = false;
    }

    void QCASim::Run()
    {
        m_Running = true;
        while (m_Running) {

        }
    }

    void QCASim::Shutdown()
    {

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