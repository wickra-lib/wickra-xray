// The scrubber path through the C++ wrapper.
//
// Same three properties scrub_test.c checks, stated through wickra_xray.hpp
// instead of the raw entry points. That is the point of running both: the C test
// proves the ABI, this one proves the wrapper over it -- the owned handle, the
// two-call length protocol it hides, and that a failure arrives as an exception
// rather than as a negative integer a caller can ignore.
#include <exception>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

#include "wickra_xray.hpp"

#include "golden_specs.h" // GOLDEN_DIR

namespace {

constexpr long MID = 12000;
constexpr long END = 24000;

std::string slurp(const std::string &path) {
    std::ifstream file(path, std::ios::binary);
    if (!file) {
        throw std::runtime_error("cannot open " + path);
    }
    std::ostringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

/// Sum every number in the frame's footprint volume arrays, the same way
/// scrub_test.c does: what is compared is one total against another produced
/// identically, so no JSON parser is needed to make the comparison meaningful.
double volume_total(const std::string &frame) {
    static const char *KEYS[] = {"\"buy_vol\":[", "\"sell_vol\":["};
    double total = 0.0;
    for (const char *key : KEYS) {
        std::size_t at = 0;
        while ((at = frame.find(key, at)) != std::string::npos) {
            at += std::string(key).size();
            const std::size_t close = frame.find(']', at);
            std::istringstream numbers(frame.substr(at, close - at));
            std::string field;
            while (std::getline(numbers, field, ',')) {
                total += std::stod(field);
            }
            at = close;
        }
    }
    return total;
}

} // namespace

int main() {
    try {
        const std::string spec = slurp(std::string(GOLDEN_DIR) + "/specs/multi_panel.json");
        const std::string dataset = slurp(std::string(GOLDEN_DIR) + "/data.json");

        wickra::Xray xray(spec);
        xray.command(R"({"cmd":"load","dataset":)" + dataset + "}");

        const std::string full = xray.command(R"({"cmd":"frame"})");
        const std::string at_end =
            xray.command(R"({"cmd":"frame_at","ts":)" + std::to_string(END) + "}");
        const std::string at_mid =
            xray.command(R"({"cmd":"frame_at","ts":)" + std::to_string(MID) + "}");

        int failures = 0;
        if (full != at_end) {
            std::cerr << "frame_at(" << END << ") is not the full frame\n"
                      << "  frame:    " << full << "\n  frame_at: " << at_end << "\n";
            failures++;
        }
        if (at_mid.find("\"cursor_ts\":" + std::to_string(MID)) == std::string::npos) {
            std::cerr << "frame_at(" << MID << ") does not report that cursor:\n  "
                      << at_mid << "\n";
            failures++;
        }
        const double mid_volume = volume_total(at_mid);
        const double end_volume = volume_total(at_end);
        if (!(mid_volume < end_volume)) {
            std::cerr << "folding to " << MID << " carried " << mid_volume << " of the "
                      << end_volume << " total volume; a midpoint that folds everything "
                      << "means the cursor is not clipping the window\n";
            failures++;
        }
        if (failures != 0) {
            return 1;
        }
        std::cout << "scrubber holds through the C++ wrapper: frame_at(" << END
                  << ") == frame, frame_at(" << MID << ") stops at its cursor ("
                  << mid_volume << " of " << end_volume << " volume)\n";
    } catch (const std::exception &e) {
        std::cerr << "wickra-xray: " << e.what() << "\n";
        return 1;
    }
    return 0;
}
