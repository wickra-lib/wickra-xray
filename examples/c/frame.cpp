// A minimal C++ example: build a frame through the wickra-xray C ABI.
#include <cstddef>
#include <iostream>
#include <string>
#include <vector>

#include "wickra_xray.h"

namespace {
const char *SPEC =
    R"({"dataset_ref":"m","symbol":"AAA","panels":[)"
    R"({"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]})";

const char *LOAD =
    R"({"cmd":"load","dataset":{"trades":[)"
    R"({"ts":1000,"price":100.4,"qty":2.0,"side":"buy"},)"
    R"({"ts":1400,"price":101.8,"qty":0.5,"side":"sell"}]}})";

const char *FRAME = R"({"cmd":"frame"})";

// Length-out protocol: learn the length, then read into a caller buffer.
std::string run(WickraXray *xray, const char *cmd) {
    int len = wickra_xray_command(xray, cmd, nullptr, 0);
    if (len < 0) {
        std::cerr << "command failed: code " << len << "\n";
        return {};
    }
    std::vector<char> buf(static_cast<std::size_t>(len) + 1);
    wickra_xray_command(xray, cmd, buf.data(),
                        static_cast<std::size_t>(buf.size()));
    return std::string(buf.data());
}
}  // namespace

int main() {
    WickraXray *xray = wickra_xray_new(SPEC);
    if (xray == nullptr) {
        std::cerr << "failed to build xray\n";
        return 1;
    }

    std::string loaded = run(xray, LOAD);
    std::string frame = run(xray, FRAME);

    std::cout << "wickra-xray " << wickra_xray_version() << "\n";
    std::cout << "loaded: " << loaded << "\n";
    std::cout << "frame: " << frame << "\n";

    wickra_xray_free(xray);
    return 0;
}
