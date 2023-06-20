#include "StatsFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>
#include <QCASim/UI/Graphics.h>

namespace QCAS {

	void StatsFrame::Render()
	{
		ImGui::Begin("Stats");

		ImGui::Text("Frame rate:");
		ImGui::SameLine();
		ImGui::PushFont(m_AppContext.GetGraphics().GetFontManager().GetBoldFont());
		ImGui::Text("%f fps", 123.0f);
		ImGui::PopFont();

		ImGui::Text("Frame time:");
		ImGui::SameLine();
		ImGui::Text("%f ms", 6.666f);

		ImGui::End();
	}

}
