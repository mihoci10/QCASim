#pragma once

namespace QCAS{

    class IFrame {
    public:
        virtual ~IFrame() {};

        virtual void Render() = 0;
    };

}