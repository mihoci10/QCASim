#pragma once

#include <QCASim/QCASimComponent.hpp>

namespace QCAS{

    class MachineStats : public QCASimComponent {
    public:
        MachineStats(const QCASim& app);
        ~MachineStats();

        double GetFrameRate() const { return m_FrameRate; };
        double GetFrameTime() const { return m_FrameTime; };
        double GetElapsedTime() const { return m_ElapsedTime; };

    private:
        void StartFrame();
        void EndFrame();

        std::chrono::time_point<std::chrono::system_clock> m_StartFrameTime, m_StopFrameTime;

        double m_FrameRate = 0;
        double m_FrameTime = 0;
        double m_ElapsedTime = 0;

        friend class QCASim;
    };
    
}