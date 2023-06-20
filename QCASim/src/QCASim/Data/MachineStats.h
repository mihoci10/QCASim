#pragma once

#include <QCASim/AppContext.hpp>

namespace QCAS{

    class MachineStats {
    public:
        MachineStats(const AppContext& appContext);
        ~MachineStats();

        double GetFrameRate() const { return m_FrameRate; };
        double GetFrameTime() const { return m_FrameTime; };

    private:
        void StartFrame();
        void EndFrame();

        std::chrono::time_point<std::chrono::system_clock> m_StartFrameTime, m_StopFrameTime;

        double m_FrameRate = 0;
        double m_FrameTime = 0;

        friend class QCASim;
    };
    
}