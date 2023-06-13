#include "MenuBarFrame.h"

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS {

	void MenuBarFrame::Render()
	{
        if (ImGui::BeginMainMenuBar())
        {
            if (ImGui::BeginMenu("File"))
            {
                ImGui::EndMenu();
            }

            if (ImGui::BeginMenu("Edit"))
            {
                ImGui::EndMenu();
            }
            
            if (ImGui::BeginMenu("View"))
            {
                ImGui::EndMenu();
            }

            ImGui::EndMainMenuBar();
        }

	}

}
