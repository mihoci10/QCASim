#pragma once

#include <nlohmann/json.hpp>

using json = nlohmann::json;

namespace QCAC::Util{

    class JsonNode {
        friend class JsonFile;

    public:
        JsonNode() = delete;
        JsonNode(json hnd) : m_Hnd(hnd) {};

    private:
        json Get() { return m_Hnd; };
        json m_Hnd;
    };
    
}