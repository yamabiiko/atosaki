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
	cwd, // exe
	cwd, // cmdline
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

std::string get_proc_cwd(int pid) {
    std::ostringstream path;
    path << "/proc/" << pid << "/cwd";

    // Buffer to store the symbolic link target
    char cwd[PATH_MAX];
    ssize_t len = readlink(path.str().c_str(), cwd, sizeof(cwd) - 1);
    if (len == -1) {
        throw std::runtime_error("Unable to read symbolic link: " + path.str());
    }

    // Null-terminate the buffer since readlink does not do it
    cwd[len] = '\0';

    return std::string(cwd);
}

std::string get_exe_fullpath(int pid) {
    std::ostringstream path;
    path << "/proc/" << pid << "/exe";

    // Buffer to store the symbolic link target
    char exe_path[PATH_MAX];
    ssize_t len = readlink(path.str().c_str(), exe_path, sizeof(exe_path) - 1);
    if (len == -1) {
        throw std::runtime_error("Unable to read symbolic link: " + path.str());
    }

    // Null-terminate the buffer since readlink does not do it
    exe_path[len] = '\0';

    return std::string(exe_path);
}

std::string get_cmdline(int pid) {
    std::ostringstream path;
    path << "/proc/" << pid << "/cmdline";

    std::ifstream file(path.str(), std::ios::in | std::ios::binary);
    if (!file.is_open()) {
        throw std::runtime_error("Unable to open: " + path.str());
    }

    // Read the entire content of the file
    std::string content((std::istreambuf_iterator<char>(file)), std::istreambuf_iterator<char>());

    // Replace null characters with spaces for readability
    for (char& c : content) {
        if (c == '\0') {
            c = ' ';
        }
    }

    // Trim trailing spaces, if any
    if (!content.empty() && content.back() == ' ') {
        content.pop_back();
    }

    return content;
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
	std::string dispatch;
	if(window.shell_id) {
	    std::string exe;
	    bool found = false;
            for(auto & cmd_entry: m_config.m_cmdEntries) {
               std::regex s(cmd_entry.exe);
	       if(std::regex_search(window.exe, s)) {
	           if(!cmd_entry.restore_cmd.empty()) {
		     exe = cmd_entry.restore_cmd;
		     found = true;
		   }
	       }
	    }
	    if(!found) {
	        dispatch = std::format("alacritty --working-directory {} -e bash -c '{} ; bash -i'", window.cwd,  window.cmdline);
	    } else {
    		std::regex pattern("\\$\\$shell_id\\$\\$");
		exe = std::regex_replace(exe, pattern, std::to_string(window.shell_id));
	        dispatch = std::format("alacritty --working-directory {} -e bash -c '{} ; bash -i'", window.cwd, exe);
	    }
	} else {
	    bool found = false;
	    std::string exe;
            for(auto & app_entry: m_config.m_appEntries) {
                std::regex c(app_entry.aClass);
                std::regex t(app_entry.aTitle);
                if (std::regex_search(window.wClass, c) && std::regex_search(window.wTitle, t)) {
                    exe = app_entry.restore_cmd;
		    found = true;
                }
            }
	    if (found) {
	        dispatch = exe;
	    } else {
		dispatch = std::format("cd {}; {} {}", window.cwd, window.exe, window.cmdline);
	    }
	}
        HyprlandAPI::invokeHyprctlCommand("dispatch",
            std::format("exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo;] {}",
	        window.workspace, window.size[0], window.size[1], window.at[0], window.at[1], dispatch));
        Debug::log(LOG, std::format("exec [workspace {} silent; float; size {}, {}; move {}, {}; pseudo; {}",
	    window.workspace, window.size[0], window.size[1], window.at[0], window.at[1], dispatch));
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
        auto cmds = config["cmds"].as_array();
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
	for (const auto& cmd: *cmds) {
	    if (const auto *table = cmd.as_table()) {
	       auto exe = table->get_as<std::string>("exe");
               auto save_cmd = table->get_as<std::string>("save_cmd");
               auto restore_cmd = table->get_as<std::string>("restore_cmd");

               auto new_entry = TerminalEntry { exe->get(), save_cmd->get(), restore_cmd->get() };

               m_config.m_cmdEntries.emplace_back(new_entry);
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

    	if(std::regex_search(window.initialClass, e)) {
    	    auto shell_pid = get_child_pids(window.pid);
	    window.shell_id = shell_pid.front();
	    for(auto & pid_s : shell_pid) {
	        window.cwd = get_proc_cwd(pid_s);
    	    	auto children = get_child_pids(pid_s);
	    	for(auto & pid_c : children) {
		   window.cwd = get_proc_cwd(pid_c);
		   window.exe = get_exe_fullpath(pid_c);
		   window.cmdline = get_cmdline(pid_c);
        	   Debug::log(LOG, std::format("PID {}", window.cmdline));
        	   Debug::log(LOG, std::format("EXE {}", window.exe));
        	   for(auto & cmd_entry: m_config.m_cmdEntries) {
            	       std::regex s(cmd_entry.exe);
		       if(std::regex_search(window.exe, s)) {
    			   std::regex pattern("\\$\\$shell_id\\$\\$");
                	   std::system(std::regex_replace(cmd_entry.save_cmd, pattern, std::to_string(window.shell_id)).c_str());
		       }
		   }
		}
	    }
    	} else {
	    bool found = false;
            for(auto & app_entry: m_config.m_appEntries) {
                std::regex c(app_entry.aClass);
                std::regex t(app_entry.aTitle);
                if (std::regex_search(window.wClass, c) && std::regex_search(window.wTitle, t)) {
                    std::system(app_entry.save_cmd.c_str());
		    found = true;
		    break;
                }
            }

	   window.cwd = get_proc_cwd(window.pid);
	   window.exe = get_exe_fullpath(window.pid);
	   window.cmdline = get_cmdline(window.pid);
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
