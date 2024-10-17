// use super::ExtensionSettings;
use regex::Regex;
use std::io::Error;
use std::process::Command;
use std::sync::LazyLock;

pub static JAVA_ABSOLUTE_PATH_TAG: &'static str = "java_absolute_path";
pub static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"java\.version\s*=\s*(?:11|17|21)")
        .expect("Failed to compile regex pattern. Aborting.")
});

// impl<'a> ExtensionSettings<'a> {
//     pub fn get_java_path(&self) -> Option<String> {
//         Some(String::from(
//             "/Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home/bin/java",
//         ))
//     }
// }

fn get_java_properties(command: String) -> Result<String, Error> {
    match Command::new(command)
        .args(vec!["-XshowSettings:properties", "-version"])
        .output()
    {
        Ok(output) => match String::from_utf8(output.stderr) {
            Ok(utf8_string) => Result::Ok(utf8_string),
            Err(from_utf8_error) => Result::Err(Error::other(from_utf8_error)),
        },
        Err(error) => Result::Err(error),
    }
}

fn is_supported_java_version(java_properties: &String) -> bool {
    REGEX.is_match(java_properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_a_supported_java_version() {
        assert_eq!(
            true,
            is_supported_java_version(&String::from(GOOD_JAVA_PROPERTIES))
        );
    }

    #[test]
    fn it_is_not_a_supported_java_version() {
        assert_eq!(
            false,
            is_supported_java_version(&String::from(BAD_JAVA_PROPERTIES))
        );
    }

    static GOOD_JAVA_PROPERTIES: &'static str = r#"
    Property settings:
        awt.toolkit = sun.lwawt.macosx.LWCToolkit
        file.encoding = UTF-8
        file.separator = /
        gopherProxySet = false
        java.awt.graphicsenv = sun.awt.CGraphicsEnvironment
        java.awt.printerjob = sun.lwawt.macosx.CPrinterJob
        java.class.path =
        java.class.version = 55.0
        java.home = /Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home
        java.io.tmpdir = /var/folders/_j/y26zw2296lg6b9m1byf20nhh0000gp/T/
        java.library.path = /Users/user_name/Library/Java/Extensions
            /Library/Java/Extensions
            /Network/Library/Java/Extensions
            /System/Library/Java/Extensions
            /usr/lib/java
            .
        java.runtime.name = OpenJDK Runtime Environment
        java.runtime.version = 11.0.20.1+1-LTS
        java.specification.maintenance.version = 2
        java.specification.name = Java Platform API Specification
        java.specification.vendor = Oracle Corporation
        java.specification.version = 11
        java.vendor = Azul Systems, Inc.
        java.vendor.url = http://www.azul.com/
        java.vendor.url.bug = http://www.azul.com/support/
        java.vendor.version = Zulu11.66+19-CA
        java.version = 11.0.20.1
        java.version.date = 2023-08-24
        java.vm.compressedOopsMode = Zero based
        java.vm.info = mixed mode
        java.vm.name = OpenJDK 64-Bit Server VM
        java.vm.specification.name = Java Virtual Machine Specification
        java.vm.specification.vendor = Oracle Corporation
        java.vm.specification.version = 11
        java.vm.vendor = Azul Systems, Inc.
        java.vm.version = 11.0.20.1+1-LTS
        jdk.debug = release
        jdk.vendor.version = Zulu11.66+19-CA
        line.separator = \n
        os.arch = aarch64
        os.name = Mac OS X
        os.version = 14.6.1
        path.separator = :
        sun.arch.data.model = 64
        sun.boot.library.path = /Library/Java/JavaVirtualMachines/zulu-11.jdk/Contents/Home/lib
        sun.cpu.endian = little
        sun.cpu.isalist =
        sun.io.unicode.encoding = UnicodeBig
        sun.java.launcher = SUN_STANDARD
        sun.jnu.encoding = UTF-8
        sun.management.compiler = HotSpot 64-Bit Tiered Compilers
        sun.os.patch.level = unknown
        user.country = US
        user.dir = /Users/user_name/projects/a_zed_extension
        user.home = /Users/user_name
        user.language = en
        user.name = user_name
        user.timezone =

    openjdk version "11.0.20.1" 2023-08-24 LTS
    OpenJDK Runtime Environment Zulu11.66+19-CA (build 11.0.20.1+1-LTS)
    OpenJDK 64-Bit Server VM Zulu11.66+19-CA (build 11.0.20.1+1-LTS, mixed mode)"#;

    static BAD_JAVA_PROPERTIES: &'static str = r#"
    Property settings:
        file.encoding = UTF-8
        file.separator = /
        java.class.path =
        java.class.version = 63.0
        java.home = /Library/Java/JavaVirtualMachines/jdk-19.jdk/Contents/Home
        java.io.tmpdir = /var/folders/27/lqf_bvkx0xndnkzc681t4nl00000gn/T/
        java.library.path = /Users/user_name/Library/Java/Extensions
            /Library/Java/Extensions
            /Network/Library/Java/Extensions
            /System/Library/Java/Extensions
            /usr/lib/java
            .
        java.runtime.name = Java(TM) SE Runtime Environment
        java.runtime.version = 19.0.1+10-21
        java.specification.name = Java Platform API Specification
        java.specification.vendor = Oracle Corporation
        java.specification.version = 19
        java.vendor = Oracle Corporation
        java.vendor.url = https://java.oracle.com/
        java.vendor.url.bug = https://bugreport.java.com/bugreport/
        java.version = 19.0.1
        java.version.date = 2022-10-18
        java.vm.compressedOopsMode = Zero based
        java.vm.info = mixed mode, sharing
        java.vm.name = Java HotSpot(TM) 64-Bit Server VM
        java.vm.specification.name = Java Virtual Machine Specification
        java.vm.specification.vendor = Oracle Corporation
        java.vm.specification.version = 19
        java.vm.vendor = Oracle Corporation
        java.vm.version = 19.0.1+10-21
        jdk.debug = release
        line.separator = \n
        native.encoding = UTF-8
        os.arch = aarch64
        os.name = Mac OS X
        os.version = 14.2.1
        path.separator = :
        stderr.encoding = UTF-8
        stdout.encoding = UTF-8
        sun.arch.data.model = 64
        sun.boot.library.path = /Library/Java/JavaVirtualMachines/jdk-19.jdk/Contents/Home/lib
        sun.cpu.endian = little
        sun.io.unicode.encoding = UnicodeBig
        sun.java.launcher = SUN_STANDARD
        sun.jnu.encoding = UTF-8
        sun.management.compiler = HotSpot 64-Bit Tiered Compilers
        user.country = US
        user.dir = /Users/user_name
        user.home = /Users/user_name
        user.language = en
        user.name = user_name

    java version "19.0.1" 2022-10-18
    Java(TM) SE Runtime Environment (build 19.0.1+10-21)
    Java HotSpot(TM) 64-Bit Server VM (build 19.0.1+10-21, mixed mode, sharing)"#;
}
