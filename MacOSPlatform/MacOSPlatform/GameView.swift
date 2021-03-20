//
//  GameView.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-18.
//

import Cocoa


class AppDelegate: NSObject, NSApplicationDelegate {
    private var window: NSWindow?
    private var displayLink: CVDisplayLink?
   
    // TODO: This will crash as it's running in a separate thread and will
    //  execute after AppDelegate has deinitialized.
    let displayCallback: CVDisplayLinkOutputCallback = { displayLink, inNow, inOutputTime, flagsIn, flagsOut, displayLinkContext in
        let viewController = unsafeBitCast(displayLinkContext, to: AppViewController.self)
        viewController.updateFramebuffer()
        return kCVReturnSuccess
    }
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        let clientRect = NSMakeRect(0, 0, (NSScreen.main?.frame.width ?? 100), (NSScreen.main?.frame.height ?? 100))
        window = NSWindow(
            contentRect: clientRect,
            styleMask: [.miniaturizable, .closable, .resizable, .titled],
            backing: .buffered,
            defer: false
        )
        
        window?.title = "Game"
        window?.contentViewController = AppViewController()
        window?.toolbar = AppToolbar(identifier: .init("Default"))
        window?.makeKeyAndOrderFront(nil)
        
        let error = CVDisplayLinkCreateWithActiveCGDisplays(&self.displayLink)
        guard let link = self.displayLink, kCVReturnSuccess == error else {
            NSLog("Display Link created with error: %d", error)
            return
        }

        CVDisplayLinkSetOutputCallback(
            link,
            displayCallback,
            UnsafeMutableRawPointer(Unmanaged.passUnretained((window?.contentViewController)!).toOpaque())
        )
        CVDisplayLinkStart(link)
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        CVDisplayLinkStop(self.displayLink!)
    }
    
    func applicationWillHide(_ notification: Notification) {
        CVDisplayLinkStop(self.displayLink!)
    }
    
    func applicationWillUnhide(_ notification: Notification) {
        CVDisplayLinkStart(self.displayLink!)
    }
    
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
}



class AppViewController: NSViewController, NSWindowDelegate {
    private var game: GameView?
    var world: UnsafeMutablePointer<Rust_WorldHandle>?
    var framebuffer: Rust_CFramebuffer?
    
    var dirty: Bool = true
    
    override func viewDidAppear() {
        view.window?.delegate = self
    }
    
    override var acceptsFirstResponder: Bool { get { return true } }
    func myKeyDown(with event: NSEvent) -> Bool {
        self.world?.pointee.camera = move_camera_position(self.world?.pointee.camera, -0.1, 0.0, 0.0);
        self.dirty = true
        return true
    }
    
    func createFramebuffer(width: Int, height: Int) {
        if let framebuffer = self.framebuffer {
            // framebuffer.pixels.deallocate()  // @TODO(ted): Leak somewhere...
        }
        
        self.framebuffer = Rust_CFramebuffer(
            width:  width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_Color>.allocate(capacity: width * height)
        )
        self.dirty = true
    }

    func windowDidResize(_ notification: Notification) {
        let width  = Int(view.window?.frame.size.width ?? 0)
        let height = Int(view.window?.frame.size.height ?? 0)
        createFramebuffer(width: width, height: height)
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.world = load_world(world_source())
        let width  = Int(self.view.visibleRect.width)
        let height = Int(self.view.visibleRect.height)
        createFramebuffer(width: width, height: height)
        
        NSEvent.addLocalMonitorForEvents(matching: .keyDown) {
            if self.myKeyDown(with: $0) {
               return nil
            } else {
               return $0
            }
         }
    }
    
    override func loadView() {
        let viewRect = NSMakeRect(0, 0, (NSScreen.main?.frame.width ?? 100) * 0.05, (NSScreen.main?.frame.height ?? 100) * 0.05)
        self.game = GameView(frame: viewRect)
        self.view = self.game!
    }
    
    func updateFramebuffer() {
        if (!self.dirty) {
            return
        }
                
        self.framebuffer = render(self.framebuffer!, self.world)
        self.game?.image = NSImage(framebuffer: self.framebuffer!)
        DispatchQueue.main.async {
            self.game?.setNeedsDisplay(self.game!.visibleRect)
        }
        self.dirty = false
    }
    
    override func viewWillTransition(to newSize: NSSize) {
        createFramebuffer(width: Int(newSize.width), height: Int(newSize.height))
    }
}


func world_source() -> String {
    if let filepath = Bundle.main.path(forResource: "world", ofType: "txt") {
        do {
            return try String(contentsOfFile: filepath)
        } catch {
            exit(-2)
        }
    } else {
        exit(-2)
    }
}



class GameView: NSView {
    var image: NSImage?
        
    override func draw(_ dirtyRect: NSRect) {
        super.draw(dirtyRect)
        self.image?.draw(in: dirtyRect, from: .zero, operation: .copy, fraction: 1.0)
    }
}

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



