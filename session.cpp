#include "session.hpp"

#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/desktop/Window.hpp>
#include <thread>
#include <toml++/toml.h>


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
	(uint64_t) to_add.get(),
        std::array<int, 2> {(int) to_add->m_vRealPosition.goal().x, (int)to_add->m_vRealPosition.goal().y},
	std::array<int, 2> {(int) to_add->m_vRealSize.goal().x, (int)to_add->m_vRealSize.goal().y},
	to_add->m_pMonitor,
	to_add->workspaceID(), 
	to_add->m_szClass, 
        to_add->m_szTitle,
        to_add->m_szInitialClass,
        to_add->m_szInitialTitle,
        to_add->getPID(),
        shell_id, // shell_id
        cwd, // cwd
        to_add->m_bPinned,
        to_add->isFullscreen()
    );
}

void SessionData::delWindowData(PHLWINDOW& to_del) {
    auto same_window = [to_del](const HyprWindowData& window) { return window.window_id == (uint64_t) to_del.get(); };

    auto it = std::find_if(m_hyprWindowData.begin(), m_hyprWindowData.end(), same_window);

    if (it != m_hyprWindowData.end()) {
        m_hyprWindowData.erase(it);
    }
}

void SessionData::openWindows() {
    for(auto & window: m_hyprWindowData) {
       HyprlandAPI::invokeHyprctlCommand("dispatch",
           std::format("exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo; alacritty",
	       window.workspace, window.size[0], window.size[1], window.at[0], window.at[1]));
    }
}

void SessionData::closeWindows() {
    for(auto & window: m_hyprWindowData) {
       HyprlandAPI::invokeHyprctlCommand("dispatch",
           std::format("closewindow address:{:#x}",
	       window.window_id));
    }
}
void SessionData::loadConfig() {
    Debug::log(LOG, "[kuukiyomu] loading conf");
    try {
        auto config = toml::parse_file("/home/yamabiko/.config/kuukiyomu/config.toml");
        auto title = config["terminal"].value<std::string>();
        auto apps = config["apps"].as_array();
        for (const auto& app : *apps) {
            if (const auto* table = app.as_table()) {
                auto cls = table->get_as<std::string>("class");
                auto title = table->get_as<std::string>("title");
                auto save_command = table->get_as<std::string>("save_command");

                auto new_entry = AppEntry { cls->get(), title->get(), save_command->get() };

                m_config.m_appEntries.emplace_back(new_entry);

            }
        }
    } catch (const toml::parse_error& err) {
        Debug::log(LOG, "[kuukiyomu] LOAD CONF FAILED");
        Debug::log(LOG, std::format("[kuukiyomu] err: {}", err.description()));
    }
}

void SessionData::customSave() {
    for(auto & window: m_hyprWindowData) {
        for(auto & app_entry: m_config.m_appEntries) {
            std::regex r(app_entry.aClass);
            if (std::regex_search(window.wClass, r)) {
                std::system(app_entry.save_command.c_str());
            }
        }
    }
}

// close all windows.. 
void SessionData::replaceSession() {
    this->closeWindows();
    m_hyprWindowData.clear();
}

void SessionData::printWindows() {
    for(auto & window: m_hyprWindowData) {
        Debug::log(LOG, window.wClass);
    }
}
