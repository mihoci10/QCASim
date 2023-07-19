#pragma once

#include <QCASim/UI/Graphics.h>
#include <QCASim/Input/Input.h>
#include <QCASim/Data/MachineStats.h>
#include <QCASim/UI/Frames/MainFrame.h>

namespace QCAS {

	class QCASim {
	public:
		void Startup();
		void Run();
		void Shutdown();

		inline bool ShouldRestart() const { return m_ShouldRestart; };

		const Input& GetInput() const { return *m_Input.get(); };
		const Graphics& GetGraphics() const { return *m_Graphics.get(); };
		const MachineStats& GetMachineStats() const { return *m_MachineStats.get(); };

	private:
		bool m_Running = false;
		bool m_ShouldRestart = false;

		std::unique_ptr<Input> m_Input;
		std::unique_ptr<Graphics> m_Graphics;
		std::unique_ptr<MainFrame> m_MainFrame;
		std::unique_ptr<MachineStats> m_MachineStats;
	};

}