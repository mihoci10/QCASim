#include "Graphics.h"

#include <Cherry/Utils/SDLUtils.hpp>

namespace QCAS::UI{

	void Graphics::Init()
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

		if (!SDL_InitSubSystem(SDL_INIT_VIDEO))
			throw std::exception("SDL initialization error!");

		m_windowHnd = std::shared_ptr<SDL_Window>(SDL_CreateWindow("QCASim", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
			512, 512, ctxFlag), Cherry::SDL_Deleter());

		if(!m_windowHnd)
			throw std::exception("SDL window initialization error!");

		Cherry::Renderer::Init();
	}

	void Graphics::Deinit()
	{
		Cherry::Renderer::Deinit();
		m_windowHnd.reset();

		SDL_QuitSubSystem(SDL_INIT_VIDEO);
	}
}