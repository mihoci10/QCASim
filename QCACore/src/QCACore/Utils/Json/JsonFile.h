#pragma once

#include <QCACore/Utils/Json/JsonNode.hpp>

enum class JsonFileMode {
    Read, Write
};

namespace QCAC::Util{

    class JsonFile {
    public:
        JsonFile();
        JsonFile(std::ifstream stream);
        JsonFile(const std::string& string);

        void inline SetMode(JsonFileMode mode) { m_Mode = mode; };

        JsonNode GetRootNode() { return m_Root; };
        JsonNode GetChildNode(JsonNode node, const std::string& name);
        JsonNode GetChildNode(JsonNode node, size_t index);

        template <typename T>
        void UpdateValue(JsonNode node, const std::string& name, T& value, T defaultValue);
        template <typename T>
        void WriteValue(JsonNode node, const std::string&, T value);
        template <typename T>
        void ReadValue(JsonNode node, const std::string&, T& value, T defaultValue);

    private:
        JsonFileMode m_Mode = JsonFileMode::Write;
        JsonNode m_Root;
    };

    template<typename T>
    inline void JsonFile::UpdateValue(JsonNode node, const std::string& name, T& value, T defaultValue) {
        switch (m_Mode)
        {
        case JsonFileMode::Read:
            ReadValue(node, name, value, defaultValue);
            break;
        case JsonFileMode::Write:
            ReadValue(node, name, value, defaultValue);
            break;
        }
    }

    template<typename T>
    inline void JsonFile::WriteValue(JsonNode node, const std::string& name, T value) {
        node.Get().at(name) = value;
    };

    template<typename T>
    inline void JsonFile::ReadValue(JsonNode node, const std::string& name, T& value, T defaultValue) {
        try {
            value = node.Get().at(name);
        }
        catch (...) {
            value = defaultValue;
        }
    };

}