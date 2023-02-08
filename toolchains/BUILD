load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")
load(":toolchain.bzl", "toolchain", "toolchain_alias")
load(":rust_toolchain.bzl", "rust_toolchain")
load("@prelude//toolchains:cxx.bzl", "system_cxx_toolchain")
load("@prelude//toolchains:python.bzl", "system_python_bootstrap_toolchain", "system_python_toolchain")

### NECESSARY SYSTEM TOOLCHAINS ###
system_cxx_toolchain(
    name = "cxx",
    visibility = ["PUBLIC"],
)

system_python_toolchain(
    name = "python",
    visibility = ["PUBLIC"],
)

system_python_bootstrap_toolchain(
    name = "python_bootstrap",
    visibility = ["PUBLIC"],
)

toolchain_alias(
   name = "rust",
   actual = select({
     "//:bootstrap": ":bootstrap_toolchain",
     "DEFAULT": ":normal_toolchain",
   })
)
### NECESSARY SYSTEM TOOLCHAINS ###


### RUST SETUP ###
system_rust_toolchain(
    name = "rust_bootstrap_toolchain",
    default_edition = "2021",
)

rust_toolchain(
    name = "normal_toolchain",
    rustc = ":rustc-with-bootstrap-toolchain"
)

configured_alias(
    name = "rustc-with-bootstrap-toolchain",
    actual = ":rustc",
    platform = ":boostrap-config"
)

rust_binary(
    name = "rustc",
    # Most basic binary that just invokes rustc
    srcs = ["rust_wrapper.rs"],
)

platform(
  name = "bootstrap_config",
  deps = [],
  constraint_values = [":bootstrap"],
)

constraint_setting(
    name = "bootstrap",
    visibility = ["PUBLIC"],
)

constraint_value(
    name = "bootstrap_contraint_value",
    constraint_setting = ":bootstrap",
    visibility = ["PUBLIC"],
)
### RUST SETUP ###
