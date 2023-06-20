#include "Graphics.h"

#include <Cherry/Utils/SDLUtils.hpp>
#include <QCASim/UI/Frames/MainFrame.h>

namespace QCAS{

	Graphics::Graphics(const AppContext& appContext, const std::shared_ptr<Cherry::RendererSettings>& rendererSettings)
		: m_RendererSettings(rendererSettings)
	{
		if (!m_RendererSettings)
			throw std::exception("Renderer settings need to be set before initialization!");

		Uint32 ctxFlag = 0;

		switch (m_RendererSettings->platform)
		{
		case Cherry::RendererPlatform::None:
			break;
		case Cherry::RendererPlatform::OpenGL:
			ctxFlag = SDL_WINDOW_OPENGL;
			break;
		case Cherry::RendererPlatform::Vulkan:
			ctxFlag = SDL_WINDOW_VULKAN;
			break;
		}

		if (SDL_InitSubSystem(SDL_INIT_VIDEO))
			throw std::exception("SDL initialization error!");

		m_windowHnd = std::shared_ptr<SDL_Window>(SDL_CreateWindow("QCASim", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
			512, 512, ctxFlag), Cherry::SDL_Deleter());

		if (!m_windowHnd)
			throw std::exception("SDL window initialization error!");

		m_RenderApi = Cherry::RendererAPI::Create(m_windowHnd, rendererSettings);
		m_RenderApi->Init();
		m_RenderApi->SetClearColor({ 0.5, 0.5, 0.5, 1 });

		m_ImGuiApi = Cherry::GUI::ImGuiAPI::Create();
		m_ImGuiApi->Init();

		SetupSDL();
		SetupImGui();

		m_FontManager = std::make_unique<FontManager>();

		m_Frame = std::make_unique<MainFrame>(appContext);
	}

	Graphics::~Graphics()
	{
		m_ImGuiApi->Deinit();
		m_RenderApi->Deinit();
		m_windowHnd.reset();

		SDL_QuitSubSystem(SDL_INIT_VIDEO);
	}

	void Graphics::BeginFrame()
	{
		m_ImGuiApi->NewFrame();
		m_RenderApi->Clear();
	}

	void Graphics::RenderFrame()
	{
		ImGui::PushFont(m_FontManager->GetRegularFont());
		m_Frame->Render();
		ImGui::PopFont();
	}

	void Graphics::EndFrame()
	{
		m_ImGuiApi->DrawFrame();
		SDL_GL_SwapWindow(m_windowHnd.get());
	}

	void Graphics::SetupSDL()
	{
		SDL_SetWindowResizable(m_windowHnd.get(), SDL_TRUE);
		SDL_MaximizeWindow(m_windowHnd.get());
		SDL_GL_SetSwapInterval(0);
	}

	void Graphics::SetupImGui()
	{
		ImGuiIO& io = ImGui::GetIO();
		//Prevent saving of window state
		io.IniFilename = NULL;
		
		io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;
	}
}