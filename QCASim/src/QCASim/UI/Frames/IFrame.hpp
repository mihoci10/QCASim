#pragma once

#include <QCASim/AppContext.hpp>

namespace QCAS{

    class BaseFrame {
    public:
        virtual ~BaseFrame() {};

        virtual void Render() = 0;

    protected:
        AppContext& m_AppContext;
    };

}