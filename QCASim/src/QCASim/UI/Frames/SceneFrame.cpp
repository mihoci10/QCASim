#include "SceneFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS {

	SceneFrame::SceneFrame(const AppContext& appContext) 
		: BaseFrame(appContext), m_Visual(std::make_unique<SceneVisual>(appContext)) {}

	void SceneFrame::Render()
	{
		ImGui::Begin("Scene");

		auto frameSize = ImGui::GetWindowSize();
		auto visualSize = m_Visual->GetSize();
		if (frameSize.x != visualSize.x || frameSize.y != visualSize.y)
			m_Visual->SetSize(frameSize.x, frameSize.y);

		m_Visual->Render();

		ImGui::Image(reinterpret_cast<void*>(m_Visual->GetTextureID()), frameSize);

		ImGui::End();
	}

}
