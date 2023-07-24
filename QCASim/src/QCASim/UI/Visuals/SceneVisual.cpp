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
			#version 410

			uniform mat4 u_ViewProjection;

			layout(location = 1) out vec3 nearPoint;
			layout(location = 2) out vec3 farPoint;

			vec3 gridPlane[6] = vec3[](
				vec3(1, 1, 0), vec3(-1, -1, 0), vec3(-1, 1, 0),
				vec3(-1, -1, 0), vec3(1, 1, 0), vec3(1, -1, 0)
				);
			
			vec3 UnprojectPoint(float x, float y, float z, mat4 viewProjection) {
				vec4 unprojectedPoint =  inverse(viewProjection) * vec4(x, y, z, 1.0);
				return unprojectedPoint.xyz / unprojectedPoint.w;
			}
			
			void main()
			{
				vec3 p = gridPlane[gl_VertexID];
				nearPoint = UnprojectPoint(p.x, p.y, 0.0, u_ViewProjection).xyz; 
				farPoint = UnprojectPoint(p.x, p.y, 1.0, u_ViewProjection).xyz; 
				gl_Position = vec4(p, 1.0);
			})"; 

		const std::string fragmentShader = R"(
			#version 410
			layout(location = 1) in vec3 nearPoint;
			layout(location = 2) in vec3 farPoint;
			layout(location = 0) out vec4 outColor;

			vec4 grid(vec3 fragPos3D, float scale) {
				vec2 coord = fragPos3D.xy / scale;
				vec2 derivative = fwidth(coord);
				vec2 grid = abs(fract(coord - 0.5) - 0.5) / derivative;
				float line = min(grid.x, grid.y);
				float minimumy = min(derivative.y, 1);
				float minimumx = min(derivative.x, 1);
				vec4 color = vec4(0.4, 0.4, 0.4, 1.0 - min(line, 1.0));
				return color;
			}
			void main() {
				float t = -nearPoint.z / (farPoint.z - nearPoint.z);
				vec3 fragPos3D = nearPoint + t * (farPoint - nearPoint);
				outColor = grid(fragPos3D, 100) * float(t > 0);
			})";

		m_Shader = Cherry::Shader::Create(
			app.GetGraphics().GetRendererApi().GetRendererSettings(),
			"Shader", 
			vertexShader,
			fragmentShader);

		m_Camera = std::make_unique<OrtographicCamera>(-1, 1, -1, 1);
		m_Camera->SetPosition({ 0, 0, 10 });
		m_Camera->SetRotation({ 0, 0, 0 });
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
		m_Camera->SetZoom(std::max(m_Camera->GetZoom() + m_App.GetInput().GetMouseWheelDelta() * 0.1, 0.1));

		m_Framebuffer->Bind();
		renderer.SetViewport( 0,0,m_Width,m_Height );
		renderer.SetClearColor({0,0,0,0});
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