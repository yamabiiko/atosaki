#include "globals.hpp"
#include <fstream>
#include <sys/socket.h>
#include <sys/un.h>

APICALL EXPORT std::string PLUGIN_API_VERSION() {
    return HYPRLAND_API_VERSION;
}

static void onNewWindow(PHLWINDOW to_add) {
    HyprWindowData data = {
        (uint64_t) to_add.get(),
        std::array<int, 2> {(int) to_add->m_vRealPosition.goal().x, (int)to_add->m_vRealPosition.goal().y},
	std::array<int, 2> {(int) to_add->m_vRealSize.goal().x, (int)to_add->m_vRealSize.goal().y},
	to_add->m_pMonitor,
	to_add->workspaceID(), 
	to_add->m_szClass, 
        to_add->m_szTitle,
        to_add->m_szInitialClass,
        to_add->m_szInitialTitle,
        to_add->m_bPinned,
        to_add->isFullscreen(),
        to_add->getPID(),
        0, // shell_id
        "", // cwd
	"", // exe
	"", // cmdline
    };

    g_pSessionData->sendMessage(data);
    HyprlandAPI::addNotification(PHANDLE, "status window", CHyprColor{0.2, 1.0, 0.2, 1.0}, 5000);

}

static void onWindowChange(PHLWINDOW window) {
    //HyprlandAPI::addNotification(PHANDLE, "added window", CHyprColor{0.2, 1.0, 0.2, 1.0}, 5000);
}

static void onCloseWindow(PHLWINDOW window) {

    HyprlandAPI::addNotification(PHANDLE, "removed window", CHyprColor{0.2, 1.0, 0.2, 1.0}, 5000);

}

static void loadSession(std::string args) {

    HyprlandAPI::addNotification(PHANDLE, "[kuukiyomu] loaded session successfully!", CHyprColor{0.2, 1.0, 0.2, 1.0}, 5000);
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

    g_pSessionData->connectToSocket();

    HyprlandAPI::addNotification(PHANDLE, "<kuukiyomu> plugin initialized", CHyprColor{0.2, 1.0, 0.2, 1.0}, 5000);

    const std::string socket_path = "/tmp/kuukiyomu_hypr.sock";
    int sockfd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (sockfd == -1) {
        perror("socket");
        //return false;
    }

   // Serialize the struct

    sockaddr_un addr{};
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, socket_path.c_str(), sizeof(addr.sun_path) - 1);

    if (connect(sockfd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
        perror("connect");
        close(sockfd);
        //return false;
    }

    HyprlandAPI::reloadConfig();

    return {"kuukiyomu", "A session manager plugin", "yamabiiko", "0.1"};

}

APICALL EXPORT void PLUGIN_EXIT() {}
