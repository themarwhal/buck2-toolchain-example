load("@prelude//toolchains:rust.bzl", "system_rust_toolchain")
load(":toolchain.bzl", "toolchain", "toolchain_alias")
load(":rust_toolchain.bzl", "rust_toolchain")
load("@prelude//toolchains:cxx.bzl", "system_cxx_toolchain")
load("@prelude//toolchains:python.bzl", "system_python_bootstrap_toolchain", "system_python_toolchain")
load(":platforms.bzl", "execution_platform", "host_configuration")

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
### NECESSARY SYSTEM TOOLCHAINS ###


### RUST TOOLCHAIN SETUP ###

# Pick the bootstrap toolchain or normal toolchain (from system) based on the bootstrap constraint
toolchain_alias(
   name = "rust",
   actual = select({
     "//:bootstrap": ":rust_bootstrap_toolchain",
     "DEFAULT": ":normal_toolchain",
   }),
   visibility = ["PUBLIC"],
)

system_rust_toolchain(
    name = "rust_bootstrap_toolchain",
    default_edition = "2021",
)

rust_toolchain(
    name = "normal_toolchain",
    compiler = ":rustc-with-bootstrap-toolchain",
    default_edition = "2021", #??
)

configured_alias(
    name = "rustc-with-bootstrap-toolchain",
    actual = ":rustc",
    platform = ":bootstrap_config"
)

rust_binary(
    name = "rustc",
    # Most basic binary that just execs into rustc
    srcs = ["rustc_wrapper.rs"],
    crate_root = "rustc_wrapper.rs",
)

platform(
  name = "bootstrap_config",
  # This seemed necessary? But not sure if this is the right way
  deps = [":default_platforms"],
  constraint_values = [":bootstrap"],
)

constraint_setting(
    name = "bootstrap_setting",
)

constraint_value(
    name = "bootstrap",
    constraint_setting = ":bootstrap_setting",
)

### RUST SETUP ###

### PLATFORM SETUP ###
# Copied and pasted from prelude because I didn't want to modify it.
# There's maybe a better way to plug in the default platform config into bootstrap_config above
execution_platform(
    name = "default_platforms",
    cpu_configuration = host_configuration.cpu,
    os_configuration = host_configuration.os,
    use_windows_path_separators = host_info().os.is_windows,
)
### PLATFORM SETUP ###
