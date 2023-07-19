#pragma once

#include <QCASim/QCASimComponent.hpp>

namespace QCAS{

    class BaseFrame : public QCASimComponent {
    public:
        virtual ~BaseFrame() {};

        virtual void Render() = 0;

    protected:
        BaseFrame(const QCASim& app) : QCASimComponent(app) {};
    };

}