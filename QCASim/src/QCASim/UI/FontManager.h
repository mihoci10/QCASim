#pragma once

#include <Cherry/GUI/ImGuiAPI.h>

namespace QCAS{

    class FontManager {
    public:
        FontManager();
        ~FontManager();

        inline ImFont* GetRegularFont() const { return m_RegularFont; };
        inline ImFont* GetItalicFont() const { return m_ItalicFont; };
        inline ImFont* GetBoldFont() const { return m_BoldFont; };

    private:
        ImFont *m_RegularFont, *m_ItalicFont, *m_BoldFont;
    };

}