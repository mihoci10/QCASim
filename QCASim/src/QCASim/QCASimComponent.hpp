#pragma once

namespace QCAS{

    class QCASim;
    
    class QCASimComponent {
    public:
        QCASimComponent(const QCASim& app) : m_App(app) {};
        ~QCASimComponent() = default;

    protected:
        const QCASim& m_App;
    };

}