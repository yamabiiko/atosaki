#include "session.hpp"

APICALL EXPORT std::string PLUGIN_API_VERSION() {
    return HYPRLAND_API_VERSION;
}

static void onNewWindow(PHLWINDOW window) {

    g_pSessionData->addWindowData(window);

}

static void onWindowChange(PHLWINDOW window) {
    // data is guaranteed
    //
}

static void onCloseWindow(void* self, std::any data) {
    // data is guaranteed
    const auto PWINDOW = std::any_cast<PHLWINDOW>(data);

}

APICALL EXPORT PLUGIN_DESCRIPTION_INFO PLUGIN_INIT(HANDLE handle) {
    PHANDLE = handle;

    static auto P = HyprlandAPI::registerCallbackDynamic(PHANDLE, "openWindow",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onNewWindow(std::any_cast<PHLWINDOW>(data)); });

    static auto P3 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "windowUpdateRules",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onWindowChange(std::any_cast<PHLWINDOW>(data)); });

    HyprlandAPI::reloadConfig();

    HyprlandAPI::addNotification(PHANDLE, "[kuukiyomu] Initialized successfully!", CColor{0.2, 1.0, 0.2, 1.0}, 5000);

    g_pSessionData = std::make_unique<SessionData>();

    return {"kuukiyomu", "A smooth hacky session manager plugin", "yamabiiko", "0.1"};

}
