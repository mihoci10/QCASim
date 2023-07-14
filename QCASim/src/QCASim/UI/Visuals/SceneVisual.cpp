#include "SceneVisual.h"

#include <QCASim/UI/Graphics.h>

namespace QCAS{
	SceneVisual::SceneVisual(const QCAS::AppContext& appContext) : BaseVisual(appContext)
	{
		Cherry::BufferDescriptor bufferDescriptor;
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 3, false);
		std::array<float, 18> vertices {};
		m_Buffer = Cherry::VertexBuffer::Create(vertices.data(), bufferDescriptor, 6);

		Cherry::FramebufferSpecification framebufferSpec = { 1, 1, 1, {Cherry::FramebufferTextureFormat::Color} };
		m_Framebuffer = Cherry::Framebuffer::Create(framebufferSpec);

		const std::string vertexShader = R"(
			#version 330 core
			uniform mat4 u_ViewProjection;
			vec3 gridPlane[6] = vec3[](
				vec3(1, 1, 0), vec3(-1, -1, 0), vec3(-1, 1, 0),
				vec3(-1, -1, 0), vec3(1, 1, 0), vec3(1, -1, 0)
				);
			
			void main()
			{
				gl_Position = u_ViewProjection * vec4(gridPlane[gl_VertexID], 1.0);
			})"; 

		const std::string fragmentShader = R"(
			#version 330 core
			out vec4 FragColor;
			
			void main()
			{
				FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
			})";

		m_Shader = Cherry::Shader::Create(
			"Shader", 
			vertexShader,
			fragmentShader);

		m_Camera = std::make_shared<OrtographicCamera>(-1, 1, -1, 1);
	}

	void SceneVisual::Render()
	{
		m_Framebuffer->Bind();
		m_AppContext.GetGraphics().GetRendererApi().SetViewport( 0,0,m_Width,m_Height );
		m_AppContext.GetGraphics().GetRendererApi().SetClearColor({0.3, 0.1, 0.1, 1});
		m_AppContext.GetGraphics().GetRendererApi().Clear();
		m_Shader->Bind();
		m_Shader->SetUniform("u_ViewProjection", m_Camera->GetViewProjection());
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