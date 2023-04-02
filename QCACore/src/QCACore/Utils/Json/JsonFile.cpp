#include "JsonFile.h"

namespace QCAC::Util{
    JsonFile::JsonFile()
        : m_Root(json()), m_Mode(JsonFileMode::Write)
    {
    }

    JsonFile::JsonFile(std::ifstream stream)
        : m_Root(JsonNode(json::parse(stream))), m_Mode(JsonFileMode::Read)
    {
    }

    JsonFile::JsonFile(const std::string& string)
        : m_Root(JsonNode(json::parse(string))), m_Mode(JsonFileMode::Read)
    {
    }

}