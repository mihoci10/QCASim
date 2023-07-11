#include "SceneVisual.h"

#include <QCASim/UI/Graphics.h>

namespace QCAS{
	SceneVisual::SceneVisual(const QCAS::AppContext& appContext) : BaseVisual(appContext)
	{
		Cherry::BufferDescriptor bufferDescriptor;
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 3, false);
		std::array<float, 9> vertices{
			-0.5f, -0.5f, 0.0f,
			0.5f, -0.5f, 0.0f,
			0.0f, 0.5f, 0.0f
		};
		m_Buffer = Cherry::VertexBuffer::Create(vertices.data(), bufferDescriptor, 3);

		Cherry::FramebufferSpecification framebufferSpec = { 1, 1, 1, {Cherry::FramebufferTextureFormat::Color} };
		m_Framebuffer = Cherry::Framebuffer::Create(framebufferSpec);

		const std::string vertexShader = 
			"#version 330 core\n"
			"layout(location = 0) in vec3 aPos;\n"
			"\n"
			"void main()\n"
			"{\n"
			"	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);\n"
			" }\0"; 

		const std::string fragmentShader = 
			"#version 330 core\n"
			"out vec4 FragColor;\n"
			"\n"
			"void main()\n"
			"{\n"
			"	FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);\n"
			" }\0";

		m_Shader = Cherry::Shader::Create(
			"Shader", 
			vertexShader,
			fragmentShader);
	}

	void SceneVisual::Render()
	{
		m_Framebuffer->Bind();
		m_AppContext.GetGraphics().GetRendererApi().SetViewport( 0,0,m_Width,m_Height );
		m_AppContext.GetGraphics().GetRendererApi().SetClearColor({0.3, 0.1, 0.1, 1});
		m_AppContext.GetGraphics().GetRendererApi().Clear();
		m_Shader->Bind();
		m_AppContext.GetGraphics().GetRendererApi().DrawTriangles(m_Buffer);
		m_Shader->Unbind();
		m_Framebuffer->Unbind();
	}

	void SceneVisual::SetSize(uint32_t width, uint32_t height)
	{
		BaseVisual::SetSize(width, height);
		m_Framebuffer->Resize(width, height);
	}

	uint32_t SceneVisual::GetTextureID()
	{
		return m_Framebuffer->GetColorAttachmentID();
	}
}