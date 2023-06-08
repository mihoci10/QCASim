#pragma once

#include <QCASim/UI/Graphics.h>
#include <QCASim/Input/Input.h>

namespace QCAS {

	class QCASim {
	public:
		void Startup();
		void Run();
		void Shutdown();

		inline bool ShouldRestart() const { return m_ShouldRestart; };

	private:
		bool m_Running = false;
		bool m_ShouldRestart = false;
	};

}