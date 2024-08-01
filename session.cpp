#include "session.hpp"

#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/desktop/Window.hpp>


SessionData::SessionData() {

}

SessionData::~SessionData() {
    m_windowData.clear();
}

std::vector<PHLWINDOW> SessionData::getWindowData() {
    return m_windowData;
}
    
void SessionData::updateWindow(PHLWINDOW& to_update) {

    auto same_window = [to_update](const PHLWINDOW& window) { return window.get() == to_update.get(); };

    auto it = std::find_if(m_windowData.begin(), m_windowData.end(), same_window);

    if (it != m_windowData.end()) {
        *it = to_update;
    }

}

void SessionData::addWindowData(PHLWINDOW& to_add) {
    m_windowData.push_back(to_add);
    HyprWindowData to_add_hypr = { 
	    std::array<uint16_t, 2> {to_add->m_vRealPosition.goal().x, to_add->m_vRealPosition.goal().y},
	    std::array<uint16_t, 2> {to_add->m_vRealSize.goal().x, to_add->m_vRealSize.goal().y},
	    to_add->m_iMonitorID,
	    to_add->m_pWorkspace, 
	    to_add->m_szClass, 
	    to_add->m_szTitle,
	    to_add->m_szInitialClass,
	    to_add->m_szInitialTitle,
	    to_add->getPID(),
	    0, // shell_id
	    "", // cwd
	    to_add->m_bPinned,
	    to_add->m_bIsFullscreen,
    };

    m_hyprWindowData.push_back(to_add_hypr);

}

void SessionData::delWindowData(PHLWINDOW& to_del) {
    auto same_window = [to_del](const PHLWINDOW& window) { return window.get() == to_del.get(); };

    auto it = std::find_if(m_windowData.begin(), m_windowData.end(), same_window);

    if (it != m_windowData.end()) {
        m_windowData.erase(it);
    }
}
