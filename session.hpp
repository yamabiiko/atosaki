#pragma once

#define WLR_USE_UNSTABLE

#include <hyprland/src/desktop/Window.hpp>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/array.hpp>

#include "globals.hpp"


struct HyprWindowData {
    std::array<uint16_t, 2> at;
    std::array<uint16_t, 2> size;
    uint8_t monitor;
    uint8_t workspace;
    std::string windowClass;
    std::string title;
    std::string initialClass;
    std::string initialTitle;
    int32_t pid;
    int32_t shell_id;
    std::string cwd;
    bool pinned;
    bool fullscreen;

    template<class Archive>
    void serialize(Archive & ar, const unsigned int version) {
        ar & at;
        ar & size;
        ar & monitor;
        ar & workspace;
        ar & windowClass;
        ar & title;
        ar & initialClass;
        ar & initialTitle;
        ar & pid;
        ar & shell_id;
        ar & cwd;
        ar & pinned;
        ar & fullscreen;
    }
};



class SessionData {
    friend class boost::serialization::access;

    template<class Archive>
    void serialize(Archive & ar, const unsigned int version)
    {
        ar & m_hyprWindowData;
    }

  public:
    SessionData();
    virtual ~SessionData();

    virtual std::vector<PHLWINDOW>    getWindowData();


    virtual void                       updateWindow(PHLWINDOW&);

    virtual void                       addWindowData(PHLWINDOW&);

    virtual void                       delWindowData(PHLWINDOW&);

  private:

    int version;

    std::vector<PHLWINDOW> m_windowData;
    std::vector<HyprWindowData> m_hyprWindowData;
};
