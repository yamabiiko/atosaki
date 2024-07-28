#define WLR_USE_UNSTABLE

#include "session.hpp"

APICALL EXPORT std::string PLUGIN_API_VERSION() {
    return HYPRLAND_API_VERSION;
}

static void onNewWindow(void* self, std::any data) {
    // data is guaranteed
    const auto PWINDOW = std::any_cast<PHLWINDOW>(data);

    HyprlandAPI::addNotification(
        PHANDLE,
	"hello world",
	CColor {1.0, 0.2, 0.2, 1.0},
	10000
    );

}

APICALL EXPORT PLUGIN_DESCRIPTION_INFO PLUGIN_INIT(HANDLE handle) {
    PHANDLE = handle;

    g_pGlobalState = std::make_unique<SGlobalState>();

    static auto P  = HyprlandAPI::registerCallbackDynamic(PHANDLE, "openWindow", [&](void* self, SCallbackInfo& info, std::any data) { onNewWindow(self, data); });
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onUpdateWindowRules(std::any_cast<PHLWINDOW>(data)); });

    HyprlandAPI::reloadConfig();

    HyprlandAPI::addNotification(PHANDLE, "[kuukiyomu] Initialized successfully!", CColor{0.2, 1.0, 0.2, 1.0}, 5000);

    return {"kuukiyomu", "A smooth hacky session manager plugin", "yamabiiko", "0.1"};
}
