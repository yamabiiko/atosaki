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
    HyprlandAPI::addNotification(PHANDLE, to_add->m_szTitle, CColor{0.2, 1.0, 0.2, 1.0}, 5000);
}

void SessionData::delWindowData(PHLWINDOW& to_del) {
    auto same_window = [to_del](const PHLWINDOW& window) { return window.get() == to_del.get(); };

    auto it = std::find_if(m_windowData.begin(), m_windowData.end(), same_window);

    if (it != m_windowData.end()) {
        m_windowData.erase(it);
    }
}
