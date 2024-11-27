#include "globals.hpp"
#include <fstream>

APICALL EXPORT std::string PLUGIN_API_VERSION() {
    return HYPRLAND_API_VERSION;
}

static void onNewWindow(PHLWINDOW window) {

    g_pSessionData->addWindowData(window);

    HyprlandAPI::addNotification(PHANDLE, "added window", CColor{0.2, 1.0, 0.2, 1.0}, 5000);

}

static void onWindowChange(PHLWINDOW window) {
    //g_pSessionData->addWindowData(window);

    //HyprlandAPI::addNotification(PHANDLE, "added window", CColor{0.2, 1.0, 0.2, 1.0}, 5000);
}

static void onCloseWindow(PHLWINDOW window) {
    g_pSessionData->delWindowData(window);

    HyprlandAPI::addNotification(PHANDLE, "removed window", CColor{0.2, 1.0, 0.2, 1.0}, 5000);

}

static void loadSession(std::string args) {

    g_pSessionData->replaceSession();
    std::ifstream ifs(args, std::ios::binary);
    
    {
        boost::archive::binary_iarchive ia(ifs);
        ia >> *g_pSessionData;  // Ensure g_pSessionData is initialized
    }
    g_pSessionData->openWindows();

    HyprlandAPI::addNotification(PHANDLE, "[kuukiyomu] loaded session successfully!", CColor{0.2, 1.0, 0.2, 1.0}, 5000);
}

static void saveSession(std::string args) {

    g_pSessionData->customSave();
    std::ofstream ofs(args, std::ios::binary);

    // save data to archive
    {
        boost::archive::binary_oarchive oa(ofs);
        // write class instance to archive
        //
        
        oa << *g_pSessionData;
        // archive and stream closed when destructors are called
    }

}

static void printSession(std::string args) {
    g_pSessionData->printWindows();
}


APICALL EXPORT PLUGIN_DESCRIPTION_INFO PLUGIN_INIT(HANDLE handle) {
    PHANDLE = handle;

    static auto P = HyprlandAPI::registerCallbackDynamic(PHANDLE, "openWindow",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onNewWindow(std::any_cast<PHLWINDOW>(data)); });

    static auto P2 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "closeWindow",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onCloseWindow(std::any_cast<PHLWINDOW>(data)); });

    static auto P3 = HyprlandAPI::registerCallbackDynamic(PHANDLE, "windowUpdateRules",
                                                          [&](void* self, SCallbackInfo& info, std::any data) { onWindowChange(std::any_cast<PHLWINDOW>(data)); });

    g_pSessionData = std::make_unique<SessionData>();
    bool save = HyprlandAPI::addDispatcher(PHANDLE, "kuukiyomu:save", saveSession);
    bool load = HyprlandAPI::addDispatcher(PHANDLE, "kuukiyomu:load", loadSession);
    bool print = HyprlandAPI::addDispatcher(PHANDLE, "kuukiyomu:print", printSession);

    g_pSessionData->loadConfig();

    if(save && load && print) {
    	HyprlandAPI::addNotification(PHANDLE, "<kuukiyomu> init succesfull v019", CColor{0.2, 1.0, 0.2, 1.0}, 5000);
    } else {
    	HyprlandAPI::addNotification(PHANDLE, "[kuukiyomu] some dispatcher failed", CColor{0.2, 1.0, 0.2, 1.0}, 5000);
    }

    Debug::log(LOG, "[kuukiyomu] testi");
	
    HyprlandAPI::reloadConfig();

    return {"kuukiyomu", "A session manager plugin", "yamabiiko", "0.1"};

}

APICALL EXPORT void PLUGIN_EXIT() {}
