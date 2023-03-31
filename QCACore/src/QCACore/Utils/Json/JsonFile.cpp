#include "JsonFile.h"

namespace QCAC::Util{

    JsonFile::JsonFile(std::ifstream stream) 
        : m_Root(JsonNode(json::parse(stream)))
    {
    }

    JsonFile::JsonFile(const std::string& string)
        : m_Root(JsonNode(json::parse(string)))
    {
    }

}