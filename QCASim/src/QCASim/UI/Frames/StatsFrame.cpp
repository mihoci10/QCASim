#include "StatsFrame.h"

#include <QCASim/QCASim.h>
#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS {

	void StatsFrame::Render()
	{
		ImGui::Begin("Stats");

		ImGui::Text("Frame rate:");
		ImGui::SameLine();
		ImGui::PushFont(m_App.GetGraphics().GetFontManager().GetBoldFont());
		ImGui::Text("%lf fps", m_App.GetMachineStats().GetFrameRate());
		ImGui::PopFont();

		ImGui::Text("Frame time:");
		ImGui::SameLine();
		ImGui::Text("%lf ms", m_App.GetMachineStats().GetFrameTime());

		ImGui::End();
	}

}
