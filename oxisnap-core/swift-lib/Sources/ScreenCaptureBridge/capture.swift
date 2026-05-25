import CoreGraphics
import Foundation
import ScreenCaptureKit
import SwiftRs

public class CaptureResult: NSObject {
    public let pixels: SRArray<UInt8>
    public let width: Int32
    public let height: Int32

    public init(pixels: SRArray<UInt8>, width: Int32, height: Int32) {
        self.pixels = pixels
        self.width = width
        self.height = height
    }
}

@_cdecl("capture_screen_swift")
public func capture_screen_swift() -> CaptureResult? {
    let semaphore = DispatchSemaphore(value: 0)
    var resultBuffer: CaptureResult? = nil

    Task {
        defer { semaphore.signal() }
        do {
            let content = try await SCShareableContent.current
            guard let display = content.displays.first else { return }

            let filter = SCContentFilter(display: display, excludingWindows: [])
            let config = SCStreamConfiguration()

            if let mode = CGDisplayCopyDisplayMode(display.displayID) {
                config.width = mode.pixelWidth
                config.height = mode.pixelHeight
            } else {
                config.width = display.width
                config.height = display.height
            }

            let image = try await SCScreenshotManager.captureImage(
                contentFilter: filter, configuration: config)

            guard let dataProvider = image.dataProvider,
                let data = dataProvider.data
            else { return }

            let width = image.width
            let height = image.height
            let bytesPerRow = image.bytesPerRow
            let packedBytesPerRow = width * 4

            var packedPixels = [UInt8](repeating: 0, count: packedBytesPerRow * height)

            packedPixels.withUnsafeMutableBufferPointer { bufferPtr in
                guard let baseAddress = bufferPtr.baseAddress else { return }
                for y in 0..<height {
                    let sourceLocation = y * bytesPerRow
                    let range = CFRange(location: sourceLocation, length: packedBytesPerRow)
                    CFDataGetBytes(data, range, baseAddress.advanced(by: y * packedBytesPerRow))
                }
            }

            resultBuffer = CaptureResult(
                pixels: SRArray(packedPixels),
                width: Int32(width),
                height: Int32(height)
            )

        } catch {
            print("Swift FFI Error: \(error)")
        }
    }

    semaphore.wait()
    return resultBuffer
}
