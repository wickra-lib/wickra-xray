// A minimal C++ example: build a frame through the wickra-xray C ABI.
//
// This uses the header-only C++ wrapper (`wickra_xray.hpp`) rather than calling
// the C entry points directly: the handle is owned and freed for you, the
// two-call length protocol is handled, and a failed call raises instead of
// returning a negative integer. Building this example is also what keeps the
// wrapper compiling -- a header nothing includes is a header nothing checks.
#include <exception>
#include <iostream>
#include <string>

#include "wickra_xray.hpp"

namespace {
const char *SPEC =
    R"({"dataset_ref":"m","symbol":"AAA","panels":[)"
    R"({"kind":"footprint","price_bin":1.0,"bucket_ms":60000}]})";

const char *LOAD =
    R"({"cmd":"load","dataset":{"trades":[)"
    R"({"ts":1000,"price":100.4,"qty":2.0,"side":"buy"},)"
    R"({"ts":1400,"price":101.8,"qty":0.5,"side":"sell"}]}})";

const char *FRAME = R"({"cmd":"frame"})";
}  // namespace

int main() {
    try {
        wickra::Xray xray(SPEC);

        const std::string loaded = xray.command(LOAD);
        const std::string frame = xray.command(FRAME);

        std::cout << "wickra-xray " << wickra::Xray::version() << "\n";
        std::cout << "loaded: " << loaded << "\n";
        std::cout << "frame: " << frame << "\n";
    } catch (const std::exception &e) {
        std::cerr << "wickra-xray: " << e.what() << "\n";
        return 1;
    }
    return 0;
}
