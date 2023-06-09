#include "MainFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS {
	MainFrame::MainFrame() 
		: m_MenuBarFrame(std::make_unique<MenuBarFrame>())
	{

	}
	void MainFrame::Render()
	{
		m_MenuBarFrame->Render();

        ImGui::Begin("asd");
        ImGui::End();

	}

}
