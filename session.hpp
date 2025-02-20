#pragma once

#include <hyprland/src/desktop/Window.hpp>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/array.hpp>

#define SOCKET_PATH "/tmp/kuukiyomu_hypr.sock"

struct HyprWindowData {
    uint64_t window_id;
    std::array<int32_t, 2> at;
    std::array<int32_t, 2> size;
    uint64_t monitor;
    int32_t workspace;
    std::string wClass;
    std::string wTitle;
    std::string initialClass;
    std::string initialTitle;
    bool pinned;
    bool fullscreen;
    int32_t shell_id;
    pid_t pid;
    std::string cwd;
    std::string exe;
    std::string cmdline;

   std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;

        auto append = [&](const void* data, size_t size) {
            const uint8_t* byteData = static_cast<const uint8_t*>(data);
            buffer.insert(buffer.end(), byteData, byteData + size);
        };

        append(&window_id, sizeof(window_id));
        append(&at, sizeof(at));
        append(&size, sizeof(size));
        append(&monitor, sizeof(monitor));
        append(&workspace, sizeof(workspace));

        // Serialize strings (length + data)
        auto appendString = [&](const std::string& str) {
            uint32_t len = str.size();
            append(&len, sizeof(len));
            append(str.data(), len);
        };

        appendString(wClass);
        appendString(wTitle);
        appendString(initialClass);
        appendString(initialTitle);
        append(&pinned, sizeof(pinned));
        append(&fullscreen, sizeof(fullscreen));
        append(&shell_id, sizeof(shell_id));
        append(&pid, sizeof(pid));
        appendString(cwd);
        appendString(exe);
        appendString(cmdline);

        return buffer;
    }
};

class SessionData {

  public:
    SessionData() : sock_fd(-1) {}
    ~SessionData() {
        if (sock_fd != -1) {
            close(sock_fd);
        }
    }

    bool connectToSocket();
    bool sendMessage(const HyprWindowData& data);

  private:
    int sock_fd;
    std::vector<HyprWindowData> m_hyprWindowData;
};
