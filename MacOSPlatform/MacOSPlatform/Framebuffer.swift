//
//  Framebuffer.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-20.
//

import Cocoa


extension NSImage {
    convenience init?(framebuffer: Rust_CFramebuffer) {
        let alphaInfo = CGImageAlphaInfo.premultipliedLast
        let bytesPerPixel = MemoryLayout<Rust_Color>.size
        let bytesPerRow = framebuffer.width * bytesPerPixel

        guard let providerRef = CGDataProvider(data: Data(
            bytes: framebuffer.pixels, count: framebuffer.height * bytesPerRow
        ) as CFData) else {
            return nil
        }

        guard let cgImage = CGImage(
            width: framebuffer.width,
            height: framebuffer.height,
            bitsPerComponent: 8,
            bitsPerPixel: bytesPerPixel * 8,
            bytesPerRow: bytesPerRow,
            space: CGColorSpaceCreateDeviceRGB(),
            bitmapInfo: CGBitmapInfo(rawValue: alphaInfo.rawValue),
            provider: providerRef,
            decode: nil,
            shouldInterpolate: true,
            intent: .defaultIntent
        ) else {
            return nil
        }

        self.init(cgImage: cgImage, size: NSSize(width: framebuffer.width, height: framebuffer.height))
    }
}
