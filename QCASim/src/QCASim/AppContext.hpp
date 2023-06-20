#pragma once

namespace QCAS{

    class Input;
    class Graphics;

    class AppContext {
    public:
        Input& GetInput() const { return *m_Input.get(); };
        Graphics& GetGraphics() const { return *m_Graphics.get(); };

    private:
        bool m_Initialized = false;
        std::unique_ptr<Input> m_Input;
        std::unique_ptr<Graphics> m_Graphics;

        friend class QCASim;
    };

}