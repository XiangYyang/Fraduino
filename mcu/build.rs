use cc;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    // C构建flag
    let c_flag = vec![
        "-DUSE_FULL_LL_DRIVER",
        "-DUSE_HAL_DRIVER",
        "-DSTM32L432xx",
    ];
    // 包含目录
    let inc_dir = vec![
        "./mcu/Core/Inc",
        "./mcu/Drivers/STM32L4xx_HAL_Driver/Inc",
        "./mcu/Drivers/STM32L4xx_HAL_Driver/Inc/Legacy",
        "./mcu/Drivers/CMSIS/Device/ST/STM32L4xx/Include",
        "./mcu/Drivers/CMSIS/Include",
        "./drv/inc",
    ];
    // 源文件目录
    let src_dir = vec![
        "./mcu/Core/Src",
        "./mcu/Drivers/STM32L4xx_HAL_Driver/Src",
        "./drv/src",
        "./drv",
    ];
    if select_and_test_target() {
        // 读取link script
        let link_script = read_link_script();
        // 然后把link script挪到OUT_DIR里面
        // 好让链接器找到它
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        let mut f = File::create(out.join("link.ld")).unwrap();
        f.write_all(link_script.as_bytes()).unwrap();
        // 加入linker search path
        println!("cargo:rustc-link-search={}", out.display());
        // C程序
        let mut sources: Vec<String> = Vec::new();
        for src_dir in src_dir {
            // 取得该目录下的文件
            let files = fs::read_dir(src_dir).unwrap();
            for file in files {
                let file = file.unwrap();
                let path = file.path();
                // 判断后缀名是*.c
                match path.extension() {
                    Some(ext) => {
                        if ext == "c" || ext == "s" {
                            let full_path = path.display().to_string();
                            // 告诉cargo什么时候应该重新run
                            println!("cargo:rerun-if-changed={}", full_path);
                            // 添加到sources
                            sources.push(full_path);
                        }
                    }
                    None => {}
                }
            }
        }
        // 读取makefile
        // 构建CubeMX生成的C部分代码
        let mut cc = cc::Build::new();
        for flag in c_flag {
            cc.flag(flag);
        }
        for dir in inc_dir {
            cc.include(dir);
        }
        for src in sources {
            cc.file(src);
        }
        // 把start up文件也放进去
        cc.file("./mcu/startup_stm32l432xx.s");
        // 构建, 并指定链接
        cc.compile("lib32.a");
        println!("cargo:rustc-link-lib=static={}", "32");
    } else {
        // 否则认为只是在做测试, 不会编译CubeMX生成的代码
    }
}

/// 判断Target, 并返回是否在进行交叉编译
fn select_and_test_target() -> bool {
    // 按照device选择配置
    let target = env::var("TARGET").unwrap();
    let is_embed = if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv6m");
        true
    }
    else if target.starts_with("thumbv7m-") || target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
        true
    }
    else if target.starts_with("thumbv8m") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
        true
    }
    else {
        false
    };
    if is_embed {
        if target.ends_with("-eabihf") {
            println!("cargo:rustc-cfg=has_fpu");
        }
        // 增加链接器脚本
        println!("cargo:rustc-link-arg={}", "-Tlink.ld");
    }
    //
    is_embed
}

/// 读取Link Script
fn read_link_script() -> String {
    let path = Path::new("./mcu/STM32L432KBUx_FLASH.ld");
    println!("cargo:rerun-if-changed={}", path.display().to_string());
    // 读取link script
    fs::read_to_string(path).expect("Cannot read link script")
}
