// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "ScreenCaptureBridge",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "ScreenCaptureBridge", type: .static, targets: ["ScreenCaptureBridge"])
    ],
    dependencies: [
        .package(url: "https://github.com/Brendonovich/swift-rs", from: "1.0.0")
    ],
    targets: [
        .target(
            name: "ScreenCaptureBridge",
            dependencies: [
                .product(name: "SwiftRs", package: "swift-rs")  // Kütüphaneye dahil ediyoruz
            ]
        )
    ]
)
