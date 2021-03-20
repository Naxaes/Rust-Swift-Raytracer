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
    private var typedView: MainView?
    var world: UnsafeMutablePointer<Rust_WorldHandle>?
    var framebuffer: Rust_CFramebuffer?
    
    var dirty: Bool = true
    
    override func viewDidAppear() {
        view.window?.delegate = self
    }

    func windowDidResize(_ notification: Notification) {
        let width  = Int(view.window?.frame.size.width ?? 0)
        let height = Int(view.window?.frame.size.height ?? 0)
        self.framebuffer = Rust_CFramebuffer(
            max_color_value: 255,
            width:  width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_Color>.allocate(capacity: width * height)
        )
        self.dirty = true
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.world = load_world(world_source())
        let width  = Int(self.view.visibleRect.width)
        let height = Int(self.view.visibleRect.height)
        self.framebuffer = Rust_CFramebuffer(
            max_color_value: 255,
            width: width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_Color>.allocate(capacity: width * height)
        )
        self.dirty = true
    }
    
    override func loadView() {
        let viewRect = NSMakeRect(0, 0, (NSScreen.main?.frame.width ?? 100) * 0.05, (NSScreen.main?.frame.height ?? 100) * 0.05)
        self.typedView = MainView(frame: viewRect)
        self.view = self.typedView!
    }
    
    func updateFramebuffer() {
        if (!self.dirty) {
            return
        }
        
        self.framebuffer = render(self.framebuffer!, self.world)
        self.typedView?.image = NSImage(framebuffer: self.framebuffer!)
        DispatchQueue.main.async {
            self.typedView?.setNeedsDisplay(self.typedView!.visibleRect)
        }
        self.dirty = false
    }
    
    override func viewWillTransition(to newSize: NSSize) {
        let width  = Int(newSize.width)
        let height = Int(newSize.height)
        self.framebuffer = Rust_CFramebuffer(
            max_color_value: 255,
            width:  width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_Color>.allocate(capacity: width * height)
        )
        self.dirty = true
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



class MainView: NSView {
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



