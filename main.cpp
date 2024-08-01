#include "session.hpp"
#include <fstream>

APICALL EXPORT std::string PLUGIN_API_VERSION() {
    return HYPRLAND_API_VERSION;
}

static void onNewWindow(PHLWINDOW window) {

    g_pSessionData->addWindowData(window);

}

static void onWindowChange(PHLWINDOW window) {
}

static void onCloseWindow(PHLWINDOW window) {

}

static void loadSession(std::string args) {

    std::ifstream ifs("session.bin", std::ios::binary);
    {
        boost::archive::binary_iarchive ia(ifs);
        // write class instance to archive
        //
        
        ia >> *g_pSessionData;
        // archive and stream closed when destructors are called
    }

    HyprlandAPI::addNotification(PHANDLE, "[kuukiyomu] loaded session successfully!", CColor{0.2, 1.0, 0.2, 1.0}, 5000);
}

static void saveSession(std::string args) {
    std::ofstream ofs("session.bin", std::ios::binary);

    // save data to archive
    {
        boost::archive::binary_oarchive oa(ofs);
        // write class instance to archive
        //
        
        oa << *g_pSessionData;
        // archive and stream closed when destructors are called
    }

}

static void loadSession() {
}

APICALL EXPORT PLUGIN_DESCRIPTION_INFO PLUGIN_INIT(HANDLE handle) {
    PHANDLE = handle;

    static auto P = HyprlandAPI::registerCallbackDynamic(PHANDLE, "openWindow",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onNewWindow(std::any_cast<PHLWINDOW>(data)); });

    static auto P2 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "closeWindow",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onCloseWindow(std::any_cast<PHLWINDOW>(data)); });

    static auto P3 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "windowUpdateRules",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onWindowChange(std::any_cast<PHLWINDOW>(data)); });

    HyprlandAPI::reloadConfig();

    HyprlandAPI::addDispatcher(PHANDLE, "kuukiyomu:save", saveSession);

    auto g_pSessionData = std::make_unique<SessionData>();

    return {"kuukiyomu", "A smooth hacky session manager plugin", "yamabiiko", "0.1"};

}
