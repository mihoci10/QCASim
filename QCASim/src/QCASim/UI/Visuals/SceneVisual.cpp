#include "SceneVisual.h"

#include <QCASim/QCASim.h>

namespace QCAS{
	SceneVisual::SceneVisual(const QCASim& app) : BaseVisual(app)
	{
		Cherry::BufferDescriptor bufferDescriptor;
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 3, false);
		std::array<float, 18> vertices {};
		m_Buffer = Cherry::VertexBuffer::Create(app.GetGraphics().GetRendererApi().GetRendererSettings(),
			vertices.data(), bufferDescriptor, 6);

		Cherry::FramebufferSpecification framebufferSpec = { 1, 1, 1, {Cherry::FramebufferTextureFormat::Color} };
		m_Framebuffer = Cherry::Framebuffer::Create(app.GetGraphics().GetRendererApi().GetRendererSettings(),
			framebufferSpec);

		const std::string vertexShader = R"(
			#version 330 core
			uniform mat4 u_ViewProjection;
			vec3 gridPlane[6] = vec3[](
				vec3(100, 100, 0), vec3(-100, -100, 0), vec3(-100, 100, 0),
				vec3(-100, -100, 0), vec3(100, 100, 0), vec3(100, -100, 0)
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
			app.GetGraphics().GetRendererApi().GetRendererSettings(),
			"Shader", 
			vertexShader,
			fragmentShader);

		m_Camera = std::make_unique<OrtographicCamera>(-1, 1, -1, 1);
	}

	void SceneVisual::Render()
	{
		const Cherry::RendererAPI& renderer = m_App.GetGraphics().GetRendererApi();

		if (m_App.GetInput().GetMouseKeyDown(ImGuiMouseButton_Right)) {
			ImVec2 mousePosDelta = m_App.GetInput().GetMousePositionDelta();
			auto camPos = m_Camera->GetPosition();
			camPos.x -= mousePosDelta.x / m_Camera->GetZoom();
			camPos.y += mousePosDelta.y / m_Camera->GetZoom();
			m_Camera->SetPosition(camPos);
		}
		m_Camera->SetZoom(m_Camera->GetZoom() + m_App.GetInput().GetMouseWheelDelta() * 0.1);

		m_Framebuffer->Bind();
		renderer.SetViewport( 0,0,m_Width,m_Height );
		renderer.SetClearColor({0.3, 0.1, 0.1, 1});
		renderer.Clear();
		m_Shader->Bind();
		m_Shader->SetUniform("u_ViewProjection", m_Camera->GetViewProjection());
		renderer.DrawTriangles(*m_Buffer.get());
		m_Shader->Unbind();
		m_Framebuffer->Unbind();
	}

	void SceneVisual::SetSize(uint32_t width, uint32_t height)
	{
		BaseVisual::SetSize(width, height);
		m_Framebuffer->Resize(width, height);
		m_Camera->SetView(width / -2.0f, width / 2.0f, height / -2.0f, height / 2.0f);
	}

	uint32_t SceneVisual::GetTextureID() const
	{
		return m_Framebuffer->GetColorAttachmentID();
	}
}