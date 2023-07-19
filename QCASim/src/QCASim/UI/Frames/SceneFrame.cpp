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
		ImGui::Begin("Scene");

		auto frameSize = ImGui::GetContentRegionAvail();

		auto visualSize = m_Visual->GetSize();
		if (frameSize.x != visualSize.x || frameSize.y != visualSize.y)
			m_Visual->SetSize(std::max(frameSize.x, 1.0f), std::max(frameSize.y, 1.0f));

		m_Visual->Render();

		ImGui::Image(reinterpret_cast<void*>(m_Visual->GetTextureID()), frameSize);

		ImGui::End();
	}

}
