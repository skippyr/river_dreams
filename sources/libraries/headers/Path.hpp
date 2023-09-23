#pragma once

#include "EnvironmentVariables.hpp"

#include <string>

namespace RiverDreams
{
    class Path
    {
    public:
        static std::string GetBaseName(std::string path);
        static std::string GetParentDirectory(std::string path);
        static std::string GetCurrentDirectoryAbbreviated(std::string repositoryRootDirectory);
    };
}
