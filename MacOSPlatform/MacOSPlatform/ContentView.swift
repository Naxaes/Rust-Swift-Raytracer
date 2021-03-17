import SwiftUI


extension Image {
    init?(bitmap: Rust_Bitmap) {
        let alphaInfo = CGImageAlphaInfo.premultipliedLast
        let bytesPerPixel = MemoryLayout<Rust_Color>.size
        let bytesPerRow = bitmap.width * bytesPerPixel
        
        let bitmap_height = bitmap.pixels.count / bitmap.width

        guard let providerRef = CGDataProvider(data: Data(
            bytes: bitmap.pixels.data, count: bitmap.pixels.count * bytesPerPixel
        ) as CFData) else {
            return nil
        }

        guard let cgImage = CGImage(
            width: bitmap.width,
            height: bitmap_height,
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

        self.init(decorative: cgImage, scale: 1.0, orientation: .up)
    }
}


public struct Renderer {
    public private(set) var bitmap: Rust_Bitmap
    private var i: Int = 0
    private var j: Int = 0
    private var height: Int = 0
    public init(width: Int, height: Int) {
        self.bitmap = create_bitmap(width, height)
        self.height = height
    }
}

public extension Renderer {
    mutating func draw() {
        let red   = Rust_Color(r: 255, g: 0, b: 0, a: 255)
        let green = Rust_Color(r: 0, g: 255, b: 0, a: 255)
        let blue  = Rust_Color(r: 0, g: 0, b: 255, a: 255)
        
        bitmap.pixels.data[j * bitmap.width + i] = [red, green, blue][(i+j) % 3]
    
        i = i + 1
        if (i >= bitmap.width) {
            i = 0
            j += 1
            if (j >= self.height) {
                j = 0
            }
        }
    }
}


struct ContentView: View {
    @StateObject var game = Game(width: 30, height: 100)
    
    var body: some View {
        game.image?
            .resizable()
            .interpolation(.none)
            .frame(width: 200, height: 200, alignment: .center)
            .aspectRatio(contentMode: .fit)
    }
}


class Game: ObservableObject {
    @Published public var image: Image?
    private var renderer: Renderer
    private var displayLink: CVDisplayLink?
    
    let displayCallback: CVDisplayLinkOutputCallback = { displayLink, inNow, inOutputTime, flagsIn, flagsOut, displayLinkContext in
        let game = unsafeBitCast(displayLinkContext, to: Game.self)
        game.renderer.draw()
        DispatchQueue.main.async {
            game.image = Image(bitmap: game.renderer.bitmap)
        }
        return kCVReturnSuccess
    }
    
    init(width: Int, height: Int) {
        self.renderer = Renderer(width: width, height: height)
        self.image = Image(bitmap: self.renderer.bitmap)

        let error = CVDisplayLinkCreateWithActiveCGDisplays(&self.displayLink)
        guard let link = self.displayLink, kCVReturnSuccess == error else {
            NSLog("Display Link created with error: %d", error)
            return
        }

        CVDisplayLinkSetOutputCallback(link, displayCallback, UnsafeMutableRawPointer(Unmanaged.passUnretained(self).toOpaque()))
        CVDisplayLinkStart(link)
    }
    
    deinit {
        CVDisplayLinkStop(self.displayLink!)
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
