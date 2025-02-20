#include "session.hpp"

#include <hyprland/src/Compositor.hpp>
#include <hyprland/src/desktop/Window.hpp>
#include <thread>
#include <regex>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>


bool SessionData::connectToSocket() {
    this->sock_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (this->sock_fd == -1) {
    	Debug::log(LOG, std::format("LINE 15"));
        return false;
    }

    sockaddr_un addr{};
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCKET_PATH, sizeof(addr.sun_path) - 1);

    if (connect(this->sock_fd, (struct sockaddr*)&addr, sizeof(addr)) == -1) {
    	Debug::log(LOG, std::format("LINE 24"));
        close(this->sock_fd);
        this->sock_fd = -1;
        return false;
    }

    Debug::log(LOG, std::format("CONNECTED"));
    return true;
}

bool SessionData::sendMessage(const HyprWindowData& data) {
    if (this->sock_fd == -1) {
        std::cerr << "Socket is not connected!" << std::endl;

    	Debug::log(LOG, std::format("SOCKET NOT CONNECT"));
	return false;
    }



    std::vector<uint8_t> buffer = data.serialize();

    ssize_t sent = send(sock_fd, buffer.data(), buffer.size(), 0);
    if (sent == -1) {
    	Debug::log(LOG, std::format("SEND ERROR ?"));
        return false;
    }

    Debug::log(LOG, std::format("SENT DAYO"));
    return true;
}
