#pragma once

#include <QCASim/AppContext.hpp>

namespace QCAS{

    class BaseFrame {
    public:
        virtual ~BaseFrame() {};

        virtual void Render() = 0;

    protected:
        BaseFrame(const AppContext& appContext) : m_AppContext(appContext) {};
        const AppContext& m_AppContext;
    };

}