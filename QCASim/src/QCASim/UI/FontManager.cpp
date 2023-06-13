#include "FontManager.h"

namespace QCAS{

	FontManager::FontManager() 
	{
		ImGuiIO& io = ImGui::GetIO();

		m_RegularFont = io.Fonts->AddFontFromFileTTF("resources/fonts/Roboto-Regular.ttf", 16);
		m_ItalicFont = io.Fonts->AddFontFromFileTTF("resources/fonts/Roboto-Italic.ttf", 16);
		m_BoldFont = io.Fonts->AddFontFromFileTTF("resources/fonts/Roboto-Bold.ttf", 16);
	}

	FontManager::~FontManager() 
	{

	}

}