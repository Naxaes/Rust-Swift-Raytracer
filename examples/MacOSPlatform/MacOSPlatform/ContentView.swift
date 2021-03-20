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
    private var world: UnsafeMutablePointer<Rust_WorldHandle>
    private var height: Int = 0
    private var width: Int = 0
    public init(width: Int, height: Int) {
        if let filepath = Bundle.main.path(forResource: "world", ofType: "txt") {
            do {
                let contents = try String(contentsOfFile: filepath)
                self.world = load_world(contents)!
            } catch {
                // contents could not be loaded
                exit(1)
            }
        } else {
            // example.txt not found!
            exit(1)
        }
        
        self.width  = width
        self.height = height
        self.bitmap = render(self.width, self.height, self.world)
    }
}

public extension Renderer {
    mutating func draw() {
        self.bitmap = render(self.width, self.height, self.world)
    }
}


struct ContentView: View {
    @StateObject var game = Game(width: 400, height: 225)
    
    var body: some View {
        game.image?
            .resizable()
            .interpolation(.none)
            .frame(width: 400, height: 225, alignment: .center)
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
        return;
        
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
