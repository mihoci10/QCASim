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
        size_t GetChildCount(JsonNode node);

        template <typename K, typename T>
        void UpdateValue(JsonNode node, const K& name, T& key, T defaultValue);
        template <typename K, typename T>
        void WriteValue(JsonNode node, const K&, T key);
        template <typename K, typename T>
        void ReadValue(JsonNode node, const K&, T& key, T defaultValue);

    private:
        JsonFileMode m_Mode = JsonFileMode::Write;
        JsonNode m_Root;
    };

    template <typename K, typename T>
    inline void JsonFile::UpdateValue(JsonNode node, const K& key, T& value, T defaultValue) {
        switch (m_Mode)
        {
        case JsonFileMode::Read:
            ReadValue(node, key, value, defaultValue);
            break;
        case JsonFileMode::Write:
            WriteValue(node, key, value);
            break;
        }
    }

    template <typename K, typename T>
    inline void JsonFile::WriteValue(JsonNode node, const K& key, T value) {
        node.Get().at(key) = value;
    };

    template <typename K, typename T>
    inline void JsonFile::ReadValue(JsonNode node, const K& key, T& value, T defaultValue) {
        try {
            value = node.Get().at(key);
        }
        catch (...) {
            value = defaultValue;
        }
    };

}