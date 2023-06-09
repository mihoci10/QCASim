#include "Graphics.h"

#include <Cherry/Utils/SDLUtils.hpp>
#include <QCASim/UI/Frames/MainFrame.h>

namespace QCAS{

	Graphics* Graphics::s_Graphics = nullptr;

	void Graphics::Initialize(const std::shared_ptr<Cherry::RendererSettings>& rendererSettings)
	{
		if (s_Graphics)
			throw std::exception("Graphics module has already been initialized!");

		s_Graphics = new Graphics(rendererSettings);
	}

	void Graphics::Deinitialize()
	{
		if (!s_Graphics)
			throw std::exception("Graphics module was not initialized!");

		delete s_Graphics;
		s_Graphics = nullptr;
	}

	Graphics& Graphics::GetInstance()
	{
		if (!s_Graphics)
			throw std::exception("Graphics module was not initialized!");

		return *s_Graphics;
	}

	void Graphics::BeginFrame()
	{
		m_ImGuiApi->NewFrame();
		m_RenderApi->Clear();
	}

	void Graphics::RenderFrame()
	{
		m_Frame->Render();
	}

	void Graphics::EndFrame()
	{
		m_ImGuiApi->DrawFrame();
		SDL_GL_SwapWindow(m_windowHnd.get());
	}

	Graphics::Graphics(const std::shared_ptr<Cherry::RendererSettings>& rendererSettings)
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

		if(!m_windowHnd)
			throw std::exception("SDL window initialization error!");

		SDL_SetWindowResizable(m_windowHnd.get(), SDL_TRUE);
		SDL_MaximizeWindow(m_windowHnd.get());

		m_RenderApi = Cherry::RendererAPI::Create(m_windowHnd, rendererSettings);
		m_RenderApi->Init();
		m_RenderApi->SetClearColor({ 0.5, 0.5, 0.5, 1 });

		m_ImGuiApi = Cherry::GUI::ImGuiAPI::Create();
		m_ImGuiApi->Init();

		//Prevent saving of window state
		ImGui::GetIO().IniFilename = NULL;

		m_Frame = std::make_unique<MainFrame>();
	}

	Graphics::~Graphics()
	{
		m_ImGuiApi->Deinit();
		m_RenderApi->Deinit();
		m_windowHnd.reset();

		SDL_QuitSubSystem(SDL_INIT_VIDEO);
	}
}