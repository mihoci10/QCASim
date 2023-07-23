#include "SceneFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS {

	SceneFrame::SceneFrame(const QCASim& app) :
		BaseFrame(app) 
	{
		m_Visual = std::make_unique<SceneVisual>(app);
	}

	void SceneFrame::Render()
	{
		auto io = ImGui::GetIO();

		ImGui::Begin("Scene");

		auto frameSize = ImGui::GetContentRegionAvail();

		auto visualSize = m_Visual->GetSize();
		if (frameSize.x != visualSize.x || frameSize.y != visualSize.y)
			m_Visual->SetSize(std::max(frameSize.x, 1.0f), std::max(frameSize.y, 1.0f));

		m_Visual->Render();

		ImVec2 pos = ImGui::GetCursorScreenPos();
		ImGui::GetWindowDrawList()->AddImage(
			reinterpret_cast<void*>(m_Visual->GetTextureID()),
			ImVec2(pos.x, pos.y),
			ImVec2(pos.x + frameSize.x, pos.y + frameSize.y),
			ImVec2(0, 1),
			ImVec2(1, 0)
		);

		ImGui::End();
	}

}
