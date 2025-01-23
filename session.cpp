#include "session.hpp"

#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/desktop/Window.hpp>
#include <thread>
#include <regex>
#include <toml++/toml.h>


SessionData::~SessionData() {
    m_hyprWindowData.clear();
}

//std::vector<PHLWINDOW> SessionData::getWindowData() {
//   return m_windowData;
//}
    
void SessionData::updateWindow(PHLWINDOW& to_update) {
    int shell_id = 0;
    std::string cwd = "";

    auto same_window = [to_update](const HyprWindowData& window) { return window.window_id == (uint64_t) to_update.get(); };
    auto it = std::find_if(m_hyprWindowData.begin(), m_hyprWindowData.end(), same_window);

    if (it != m_hyprWindowData.end()) {
        *it =    HyprWindowData {
	(uint64_t) to_update.get(),
        std::array<int, 2> {(int) to_update->m_vRealPosition.goal().x, (int)to_update->m_vRealPosition.goal().y},
	std::array<int, 2> {(int) to_update->m_vRealSize.goal().x, (int)to_update->m_vRealSize.goal().y},
	to_update->m_pMonitor,
	to_update->workspaceID(), 
	to_update->m_szClass, 
        to_update->m_szTitle,
        to_update->m_szInitialClass,
        to_update->m_szInitialTitle,
        to_update->getPID(),
        shell_id, // shell_id
        cwd, // cwd
        to_update->m_bPinned,
        to_update->isFullscreen()
        };
    }
}

std::vector<int> get_child_pids(int pid) {
    std::vector<int> children;
    std::ostringstream path;
    path << "/proc/" << pid << "/task/" << pid << "/children";

    std::ifstream file(path.str());
    if (!file.is_open()) {
        throw std::runtime_error("Unable to open: " + path.str());
    }

    int child_pid;
    while (file >> child_pid) {
        children.push_back(child_pid);
    }
    return children;
}

std::string<int> get_proc_cwd(int pid) {
    std::vector<int> children;
    std::ostringstream path;
    path << "/proc/" << pid << "/task/" << pid << "/children";

    std::ifstream file(path.str());
    if (!file.is_open()) {
        throw std::runtime_error("Unable to open: " + path.str());
    }

    int child_pid;
    while (file >> child_pid) {
        children.push_back(child_pid);
    }
    return children;
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
	cwd, // exe
	cwd, // cmdline
        to_add->m_bPinned,
        to_add->isFullscreen()
    );


    Debug::log(LOG, std::format("SAVED WINDOW size {}, {}; pos {}, {};", (int) to_add->m_vRealPosition.goal().x, (int)to_add->m_vRealPosition.goal().y,
        (int) to_add->m_vRealSize.goal().x, (int)to_add->m_vRealSize.goal().y));

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
            std::format("exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo;] alacritty",
	        window.workspace, window.size[0], window.size[1], window.at[0], window.at[1]));
        Debug::log(LOG, std::format("exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo; alacritty",
	    window.workspace, window.size[0], window.size[1], window.at[0], window.at[1]));
        for(auto & app_entry: m_config.m_appEntries) {
            std::regex c(app_entry.aClass);
            std::regex t(app_entry.aTitle);
            if (std::regex_search(window.wClass, c) && std::regex_search(window.wTitle, t)) {
                std::system(app_entry.restore_cmd.c_str());
            }
        }
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
	std::optional<std::string> terminal = config["terminal"].value<std::string>();
        auto apps = config["apps"].as_array();
	if(terminal) {
	    m_config.terminal = *terminal;
	}
        for (const auto& app : *apps) {
            if (const auto* table = app.as_table()) {
                auto cls = table->get_as<std::string>("class");
                auto title = table->get_as<std::string>("title");
                auto save_cmd = table->get_as<std::string>("save_cmd");
                auto restore_cmd = table->get_as<std::string>("restore_cmd");

                auto new_entry = AppEntry { cls->get(), title->get(), save_cmd->get(), restore_cmd->get() };

                m_config.m_appEntries.emplace_back(new_entry);

            }
        }
    } catch (const toml::parse_error& err) {
        Debug::log(LOG, "[kuukiyomu] LOAD CONF FAILED");
        Debug::log(LOG, std::format("[kuukiyomu] err: {}", err.description()));
    }
}

void SessionData::save() {
    for(auto & window: m_hyprWindowData) {
    	std::regex e(m_config.terminal);

    	if(std::regex_search(windon.initialClass, e)) {
    	    auto child_pid = get_child_pids(to_add->getPID());
	    for(auto & pid : child_pid) {
	        Debug::log(LOG, std::format("CHILD PID {}", pid));
	    }
    	}
        for(auto & app_entry: m_config.m_appEntries) {
            std::regex c(app_entry.aClass);
            std::regex t(app_entry.aTitle);
            if (std::regex_search(window.wClass, c) && std::regex_search(window.wTitle, t)) {
                std::system(app_entry.save_cmd.c_str());
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
