#include "MainFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS {
	MainFrame::MainFrame() 
		: 
		m_MenuBarFrame(std::make_unique<MenuBarFrame>()),
		m_SceneFrame(std::make_unique<SceneFrame>()), 
		m_StatsFrame(std::make_unique<StatsFrame>())
	{

	}
	void MainFrame::Render()
	{
		ImGuiViewport* viewport = ImGui::GetMainViewport();
		ImGuiIO& io = ImGui::GetIO();

		m_MenuBarFrame->Render();

		ImGuiID dockspace_id = ImGui::DockSpaceOverViewport(viewport, ImGuiDockNodeFlags_PassthruCentralNode);
		static bool init = true;
		ImGuiID dock_id_left, dock_id_right;
		if (init) {
			init = false;
			ImGui::DockBuilderRemoveNode(dockspace_id);
			ImGui::DockBuilderAddNode(dockspace_id);
			ImGui::DockBuilderSetNodeSize(dockspace_id, ImGui::GetMainViewport()->Size);

			ImGui::DockBuilderSplitNode(dockspace_id, ImGuiDir_Left, 0.8f, &dock_id_left, &dock_id_right);
			ImGui::DockBuilderDockWindow("Scene", dock_id_left);
			ImGui::DockBuilderDockWindow("Stats", dock_id_right);

			ImGui::DockBuilderFinish(dockspace_id);
		}

		m_SceneFrame->Render();
		m_StatsFrame->Render();

	}

}
