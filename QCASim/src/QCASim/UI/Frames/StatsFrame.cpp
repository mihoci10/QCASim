#include "StatsFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>
#include <QCASim/UI/Graphics.h>
#include <QCASim/Data/MachineStats.h>

namespace QCAS {

	void StatsFrame::Render()
	{
		ImGui::Begin("Stats");

		ImGui::Text("Frame rate:");
		ImGui::SameLine();
		ImGui::PushFont(m_AppContext.GetGraphics().GetFontManager().GetBoldFont());
		ImGui::Text("%lf fps", m_AppContext.GetMachineStats().GetFrameRate());
		ImGui::PopFont();

		ImGui::Text("Frame time:");
		ImGui::SameLine();
		ImGui::Text("%lf ms", m_AppContext.GetMachineStats().GetFrameTime());

		ImGui::End();
	}

}
