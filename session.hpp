#pragma once

#include <hyprland/src/desktop/Window.hpp>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/array.hpp>

struct HyprWindowData {
    uint64_t window_id;
    std::array<int, 2> at;
    std::array<int, 2> size;
    uint64_t monitor;
    int workspace;
    std::string wClass;
    std::string wTitle;
    std::string initialClass;
    std::string initialTitle;
    pid_t pid;
    int32_t shell_id;
    std::string cwd;
    std::string exe;
    std::string cmdline;
    bool pinned;
    bool fullscreen;

    template<class Archive>
    void serialize(Archive & ar, const unsigned int version) {
        ar & window_id;
        ar & at;
        ar & size;
        ar & monitor;
        ar & workspace;
        ar & wClass;
        ar & wTitle;
        ar & initialClass;
        ar & initialTitle;
        ar & pid;
        ar & shell_id;
        ar & cwd;
        ar & exe;
        ar & cmdline;
        ar & pinned;
        ar & fullscreen;
    }
};

struct AppEntry {
    std::string aClass;
    std::string aTitle;
    std::string save_cmd;
    std::string restore_cmd;
};

struct Config {
    std::vector<AppEntry> m_appEntries;
    std::string terminal;
    //std::vector<TerminalEntry> m_appEntries;
};



class SessionData {
    friend class boost::serialization::access;

    template<class Archive>
    void serialize(Archive & ar, const unsigned int version)
    {
        ar & m_hyprWindowData;
    }

  public:
    virtual ~SessionData();

    //virtual std::vector<PHLWINDOW>    getWindowData();


    virtual void                       updateWindow(PHLWINDOW&);

    virtual void                       addWindowData(PHLWINDOW&);

    virtual void                       delWindowData(PHLWINDOW&);
    virtual void 		       printWindows();
    virtual void 		       openWindows();
    virtual void 		       closeWindows();
    virtual void 		       loadConfig();
    virtual void 		       customSave();
    virtual void 		       replaceSession();

  private:

    int version;

    //std::vector<PHLWINDOW> m_windowData;
    std::vector<HyprWindowData> m_hyprWindowData;
    Config m_config;
};
