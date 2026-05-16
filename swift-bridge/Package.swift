// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "SystemConfigurationBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "SystemConfigurationBridge",
            type: .static,
            targets: ["SystemConfigurationBridge"])
    ],
    targets: [
        .target(
            name: "SystemConfigurationBridge",
            path: "Sources/SystemConfigurationBridge",
            publicHeadersPath: "include")
    ]
)
