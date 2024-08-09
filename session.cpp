#include "session.hpp"

#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/desktop/Window.hpp>
#include <thread>


SessionData::~SessionData() {
    m_hyprWindowData.clear();
}

//std::vector<PHLWINDOW> SessionData::getWindowData() {
//   return m_windowData;
//}
    
void SessionData::updateWindow(PHLWINDOW& to_update) {

    //auto same_window = [to_update](const PHLWINDOW& window) { return window.get() == to_update.get(); };
    //auto it = std::find_if(m_windowData.begin(), m_windowData.end(), same_window);

    //if (it != m_windowData.end()) {
        //*it = to_update;
    //}
}

void SessionData::addWindowData(PHLWINDOW& to_add) {

    int shell_id = 0;
    std::string cwd = "";

    m_hyprWindowData.emplace_back(
        std::array<int, 2> {(int) to_add->m_vRealPosition.goal().x, (int)to_add->m_vRealPosition.goal().y},
	std::array<int, 2> {(int) to_add->m_vRealSize.goal().x, (int)to_add->m_vRealSize.goal().y},
	to_add->m_iMonitorID,
	to_add->workspaceID(), 
	to_add->m_szClass, 
        to_add->m_szTitle,
        to_add->m_szInitialClass,
        to_add->m_szInitialTitle,
        to_add->getPID(),
        shell_id, // shell_id
        cwd, // cwd
        to_add->m_bPinned,
        to_add->m_bIsFullscreen
    );
}

void SessionData::delWindowData(PHLWINDOW& to_del) {
    //auto same_window = [to_del](const PHLWINDOW& window) { return window.get() == to_del.get(); };

    //auto it = std::find_if(m_windowData.begin(), m_windowData.end(), same_window);

    //if (it != m_windowData.end()) {
        //m_windowData.erase(it);
    //}
}

void SessionData::openWindows() {
    for(auto & window: m_hyprWindowData) {
       HyprlandAPI::invokeHyprctlCommand("dispatch",
           std::format("exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo; alacritty",
	       window.workspace, window.size[0], window.size[1], window.at[0], window.at[1]));
    }
}

void SessionData::printWindows() {
    for(auto & window: m_hyprWindowData) {
        Debug::log(LOG, window.wClass);
    }
}
