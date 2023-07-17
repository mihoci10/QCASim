#include "MachineStats.h"

namespace QCAS{

    MachineStats::MachineStats(const QCASim& app) : QCASimComponent(app)
    {
    }

    MachineStats::~MachineStats()
    {
    }

    void MachineStats::StartFrame()
    {
        m_StartFrameTime = std::chrono::system_clock::now();
    }

    void MachineStats::EndFrame()
    {
        m_StopFrameTime = std::chrono::system_clock::now();

        auto duration = std::chrono::duration<double, std::milli>(m_StopFrameTime - m_StartFrameTime);

        m_FrameTime = duration.count();
        m_FrameRate = 1000 / m_FrameTime;

        m_ElapsedTime += m_FrameTime;
    }

}