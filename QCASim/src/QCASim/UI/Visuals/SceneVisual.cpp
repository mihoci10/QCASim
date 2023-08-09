#include "SceneVisual.h"

#include <QCASim/QCASim.h>

namespace QCAS{
	SceneVisual::SceneVisual(const QCASim& app) : BaseVisual(app)
	{
		Cherry::FramebufferSpecification framebufferSpec = { 1, 1, 1, {Cherry::FramebufferTextureFormat::Color} };
		m_Framebuffer = Cherry::Framebuffer::Create(app.GetGraphics().GetRendererApi().GetRendererSettings(),
			framebufferSpec);

		m_Camera = std::make_unique<OrtographicCamera>(-1, 1, -1, 1);
		m_Camera->SetPosition({ 0, 0, 1 });
		m_Camera->SetRotation({ 0, 0, 0 });

		InitWorldGrid();
		InitCells();
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
		m_GridShader->Bind();
		m_GridShader->SetUniform("u_ViewProjection", m_Camera->GetViewProjection());
		renderer.Draw(*m_GridBuffer.get());
		m_GridShader->Unbind();
		m_CellShader->Bind();
		m_CellShader->SetUniform("u_ViewProjection", m_Camera->GetViewProjection());
		m_CellShader->SetUniform("u_Resolution", GetSize());
		m_CellShader->SetUniform("u_Zoom", m_Camera->GetZoom());
		renderer.Draw(*m_CellsBufferBatch.get());
		m_CellShader->Unbind();
		m_Framebuffer->Unbind();
	}

	void SceneVisual::SetSize(uint32_t width, uint32_t height)
	{
		BaseVisual::SetSize(width, height);
		m_Framebuffer->Resize(width, height);
		m_Camera->SetView(width / -2.0f, width / 2.0f, height / -2.0f, height / 2.0f);
	}

	void SceneVisual::InitWorldGrid()
	{
		Cherry::BufferDescriptor bufferDescriptor;
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 3, false);
		std::array<float, 18> vertices {};
		m_GridBuffer = Cherry::VertexBuffer::Create(m_App.GetGraphics().GetRendererApi().GetRendererSettings(),
			vertices.data(), bufferDescriptor, 6);

		const std::string vertexShader = R"(
			#version 450

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
			#version 450
			layout(location = 1) in vec3 nearPoint;
			layout(location = 2) in vec3 farPoint;
			layout(location = 0) out vec4 outColor;

			uniform highp vec2 u_Resolution;

			vec4 grid(vec3 fragPos3D, float scale) {
				vec2 grid = abs(fract(fragPos3D.xy / scale - 0.5) - 0.5);
				vec2 line = vec2(1.0) - smoothstep(vec2(0.0), vec2(0.01), grid);
				return vec4(0.3, 0.3, 0.3, min(max(line.x, line.y), 1.0));
			}
			void main() {
				float t = -nearPoint.z / (farPoint.z - nearPoint.z);
				vec3 fragPos3D = nearPoint + t * (farPoint - nearPoint);
				outColor = grid(fragPos3D, 100) * float(t > 0);
			})";

		m_GridShader = Cherry::Shader::Create(
			m_App.GetGraphics().GetRendererApi().GetRendererSettings(),
			"Shader",
			vertexShader,
			fragmentShader);
	}

	void SceneVisual::InitCells()
	{
		Cherry::BufferDescriptor bufferDescriptor;
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 3, false);
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 3, false);
		bufferDescriptor.AddSegment(Cherry::BufferDataType::FLOAT, 4, true);
		std::array<CellData, 4> vertices {
			CellData{ {0, 0, 0}, { -1, -1, 0 }, { 1, 0.5, 0, 1 } },
			CellData{ {100, 0, 0}, { 1, -1, 0 }, { 1, 0.5, 0, 1 } },
			CellData{ {100, 100, 0}, { 1, 1, 0 }, { 1, 0.5, 0, 1 } },
			CellData{ {0, 100, 0}, { -1, 1, 0 }, { 1, 0.5, 0, 1 } }
		};
		std::shared_ptr<Cherry::VertexBuffer> cellBuffer = Cherry::VertexBuffer::Create(m_App.GetGraphics().GetRendererApi().GetRendererSettings(),
			vertices.data(), bufferDescriptor, 6);

		Cherry::BufferDescriptor bufferDescriptorInd;
		bufferDescriptorInd.AddSegment(Cherry::BufferDataType::UINT_32, 1, false);
		std::array<uint32_t, 6> indices{
			0, 1, 2,
			0, 2, 3
		};
		std::shared_ptr<Cherry::IndexBuffer> indexBuffer = Cherry::IndexBuffer::Create(m_App.GetGraphics().GetRendererApi().GetRendererSettings(),
			indices.data(), bufferDescriptorInd, 6);

		m_CellsBufferBatch = Cherry::BufferBatch::Create(m_App.GetGraphics().GetRendererApi().GetRendererSettings());
		m_CellsBufferBatch->AddVertexBuffer(cellBuffer);
		m_CellsBufferBatch->SetIndexBuffer(indexBuffer);

		const std::string vertexShader = R"(
			#version 450

			struct CellData
			{
				vec3 GlobalPos;
				vec3 LocalPos;
				vec4 Color;
			};

			layout(location = 0) in vec3 aPos;
			layout(location = 1) in vec3 lPos;
			layout(location = 2) in vec4 color;

			uniform mat4 u_ViewProjection;

			layout(location = 0) out CellData cache;
			
			void main()
			{
				cache.GlobalPos = aPos;
				cache.LocalPos = lPos;
				cache.Color = color;

				gl_Position = u_ViewProjection * vec4(aPos, 1.0);
			})";

		const std::string fragmentShader = R"(
			#version 450

			struct CellData
			{
				vec3 GlobalPos;
				vec3 LocalPos;
				vec4 Color;
			};

			layout(location = 0) in CellData cache;

			layout(location = 0) out vec4 outColor;

			uniform highp vec2 u_Resolution;
			uniform highp float u_Zoom;

			float RectMask(vec2 pos, vec2 size)
			{
				vec2 startLimit = 1 - 2 * size;
				vec2 stopLimit = 1 - size;
				vec2 mask = smoothstep(startLimit, stopLimit, abs(pos));
				return max(max(mask.x, mask.y), 0.0);
			}

			void main() {
				vec2 fragSize = fwidth(cache.LocalPos.xy);

				float mask = RectMask(cache.LocalPos.xy, fragSize);
				outColor = vec4(cache.Color.rgb, mask);
			})";

		m_CellShader = Cherry::Shader::Create(
			m_App.GetGraphics().GetRendererApi().GetRendererSettings(),
			"Shader",
			vertexShader,
			fragmentShader);
	}

	uint32_t SceneVisual::GetTextureID() const
	{
		return m_Framebuffer->GetColorAttachmentID();
	}
}