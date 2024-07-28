#pragma once

#define WLR_USE_UNSTABLE

#include <hyprland/src/desktop/Window.hpp>
#include "globals.hpp"


class SessionData {
  public:
    SessionData();
    virtual ~SessionData();

    virtual std::vector<PHLWINDOW>    getWindowData();

    virtual void                       updateWindow(PHLWINDOW&);

    virtual void                       addWindowData(PHLWINDOW&);

    virtual void                       delWindowData(PHLWINDOW&);

  private:

    std::vector<PHLWINDOW> m_windowData;
};
