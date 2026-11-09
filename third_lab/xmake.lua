add_rules("mode.debug", "mode.release")
set_languages("c++20")

target("third_lab")
    set_kind("binary")
    add_files("src/*.cpp")