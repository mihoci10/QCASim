#include "FontManager.h"

namespace QCAS{

	FontManager* FontManager::s_FontManager = nullptr;

	void FontManager::Initialize()
	{
		if (s_FontManager)
			throw std::exception("Font manager has already been initialized!");

		s_FontManager = new FontManager();
	}

	void FontManager::Deinitialize()
	{
		if (!s_FontManager)
			throw std::exception("Font manager was not initialized!");

		delete s_FontManager;
		s_FontManager = nullptr;
	}

	FontManager& FontManager::GetInstance()
	{
		if (!s_FontManager)
			throw std::exception("Font manager was not initialized!");

		return *s_FontManager;
	}

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