// Wickra X-Ray — C++ wrapper over the C ABI.
//
// Header-only, C++17, no dependency beyond the standard library and
// `wickra_xray.h` beside it. Link the same `wickra_xray` library the C binding
// does.
//
// What it adds over calling the C functions directly is the handling nobody
// wants to write twice: the handle is owned and freed, the two-call length
// protocol behind `wickra_xray_command` is done for you, and a failure comes
// back as an exception rather than a negative integer a caller can ignore.
//
//     #include <wickra_xray.hpp>
//
//     wickra::Xray xray(R"({"dataset_ref":"m","symbol":"AAA","panels":[...]})");
//     xray.command(R"({"cmd":"load","dataset":{...}})");
//     std::string frame = xray.command(R"({"cmd":"frame"})");
//
// The X-Ray is data-driven, so this wrapper deliberately stops at strings: the
// spec and the frame are JSON, and which JSON library a caller uses is their
// choice, not this header's.

#ifndef WICKRA_XRAY_HPP
#define WICKRA_XRAY_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>

#include "wickra_xray.h"

namespace wickra {

/// Thrown when the library rejects a spec or a command.
class XrayError : public std::runtime_error {
 public:
  explicit XrayError(const std::string& what) : std::runtime_error(what) {}
};

/// An owning handle to an X-Ray built from a spec.
///
/// Move-only, because the underlying handle is a unique resource: copying it
/// would free the same pointer twice.
class Xray {
 public:
  /// Build an X-Ray from a spec JSON string.
  ///
  /// Throws `XrayError` if the spec is not valid JSON or not a valid spec.
  explicit Xray(const std::string& spec_json)
      : handle_(wickra_xray_new(spec_json.c_str())) {
    if (handle_ == nullptr) {
      throw XrayError("wickra_xray_new rejected the spec");
    }
  }

  ~Xray() { wickra_xray_free(handle_); }

  Xray(const Xray&) = delete;
  Xray& operator=(const Xray&) = delete;

  Xray(Xray&& other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }

  Xray& operator=(Xray&& other) noexcept {
    if (this != &other) {
      wickra_xray_free(handle_);
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }

  /// Apply a command JSON and return the response JSON.
  ///
  /// The C entry point writes into a caller buffer and reports the length it
  /// needed, so this asks for the length first and then reads. A command the
  /// library understands but cannot carry out answers in band with
  /// `{"ok":false,"error":...}`; a negative return is a failure of the call
  /// itself and becomes an exception.
  std::string command(const std::string& cmd_json) {
    const std::int32_t needed =
        wickra_xray_command(handle_, cmd_json.c_str(), nullptr, 0);
    if (needed < 0) {
      throw XrayError("wickra_xray_command failed with code " +
                      std::to_string(needed));
    }

    std::string out(static_cast<std::size_t>(needed), '\0');
    // The C side writes a trailing NUL, so the buffer has to hold one more byte
    // than the response itself.
    const std::int32_t written = wickra_xray_command(
        handle_, cmd_json.c_str(), out.data(),
        static_cast<std::uintptr_t>(out.size()) + 1);
    if (written < 0) {
      throw XrayError("wickra_xray_command failed with code " +
                      std::to_string(written));
    }
    if (written != needed) {
      // The response changed length between the two calls, which cannot happen
      // for a handle only this thread is using. Saying so is better than
      // returning a string that is half of one answer and half of another.
      throw XrayError("wickra_xray_command length changed between calls");
    }
    return out;
  }

  /// The library version.
  static std::string version() { return std::string(wickra_xray_version()); }

 private:
  WickraXray* handle_;
};

}  // namespace wickra

#endif  // WICKRA_XRAY_HPP
