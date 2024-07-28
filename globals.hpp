#pragma once

#include <hyprland/src/plugins/PluginAPI.hpp>

inline HANDLE PHANDLE = nullptr;

class SessionData;

inline std::unique_ptr<SessionData> g_pSessionData;
