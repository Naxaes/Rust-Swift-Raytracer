//
//  GameView.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-18.
//

import Cocoa


/*
 
 NSApplication -> AppDelegate
                      |
                      V
                  GameWindow -> GameWindowController
 
 
 */



class AppDelegate: NSObject, NSApplicationDelegate {
    private var window: NSWindow?
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        self.window = GameWindow()
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        print("[Application] Will terminate.")
    }
    
    func applicationWillHide(_ notification: Notification) {
        print("[Application] Will hide.")
    }
    
    func applicationWillUnhide(_ notification: Notification) {
        print("[Application] Will unhide.")
    }
    
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
}


class GameWindow : NSWindow {
    override init(contentRect: NSRect, styleMask style: NSWindow.StyleMask, backing backingStoreType: NSWindow.BackingStoreType, defer flag: Bool) {
        super.init(contentRect: contentRect, styleMask: style, backing: backingStoreType, defer: flag)
    }
    
    convenience init() {
        self.init(
            contentRect: NSMakeRect(0, 0, 200, 200),
            styleMask: [.miniaturizable, .closable, .resizable, .titled, .borderless],  // Can't have both .titled and .borderless.
            backing: .buffered,
            defer: false
        )
        
        // self.appearance = NSAppearance(named: .darkAqua)
        // self.toggleFullScreen(self)
        // self.toggleTabBar(self)       // Looks like an extra tab bar under the window title.
        // self.toggleTabOverview(self)  // Like the view when you 3-finger swipe up, but fullscreen for the app.
        // self.toggleToolbarShown(self) // Looks like toggleTabBar.
        self.title = "Game"
        self.contentViewController = GameViewController()  // TODO: Make sure this is correct.
        
        // self.toolbar = AppToolbar(identifier: .init("Default"))
        self.makeKeyAndOrderFront(nil)
        
        self.hasShadow = false
    
                
        // Make window fully transparent.
        self.isOpaque = false
        // self.alphaValue = 1.0  // NOTE: Lol! Didn't expect this. This affects the whole window though, even the GameView.
        self.backgroundColor = .clear
    }
    
    // Must be overridden for borderless style
    override var canBecomeKey:  Bool { return true }
    override var canBecomeMain: Bool { return true }
    // override var canBecomeVisibleWithoutLogin: Bool = true  //  TODO: WHAT IS THIS?
    
}



// Controls the GameView and Window.
class GameViewController: NSViewController, NSWindowDelegate {
    var game: GameView?
    var world: UnsafeMutablePointer<Rust_WorldHandle>?
    var framebuffer: Rust_CFramebuffer?
    
    var dirty: Bool = true

    override var acceptsFirstResponder: Bool { get { return true } }
    
    // ---- Update ----
    
    
    
    
    // ---- Dirty functions start ----
    func myKeyDown(with event: NSEvent) -> Bool {
        print("[Key] Key down. \(event)")
        // TODO: This must be locked or handled in some way.
        //  Changing the camera during render will give weird results.
        //  Maybe just copy it?
        self.world?.pointee.camera = move_camera_position(self.world?.pointee.camera, -0.1, 0.0, 0.0);
        self.dirty = true
        self.updateFramebuffer()
        return true
    }

    


    // ---- View initialization ----
    override func loadView() {
        print("[View] Loading.")
        self.game = GameView()  // TODO: Solve this terrible hack...
        self.view = self.game!
    }
    
    
    override func viewDidLoad() {
        print("[View] Did load.")
        super.viewDidLoad()
        self.world = load_world(world_source())

        NSEvent.addLocalMonitorForEvents(matching: .keyDown) {
            if self.myKeyDown(with: $0) {
               return nil
            } else {
               return $0
            }
        }
    }
    
    override func viewDidAppear() {
        print("[View] Did appear.")
        // A view hasn't been added to the window until it appears, so we have to do it here.
        self.view.window?.delegate = self
        
        if let screen = NSScreen.main {
            let rect   = screen.frame
            let height = rect.size.height * 0.5
            let width  = rect.size.width  * 0.5
            
            self.view.window?.setFrame(NSMakeRect(width / 2.0, height / 2.0, width, height), display: true)
            self.resizeFramebuffer()
            self.updateFramebuffer()
        }
    }
    
    

    // ---- Window ----
    func windowDidDeminiaturize(_ notification: Notification) {
        print("[Window] Did deminiaturizing.")
    }
    func windowDidMiniaturize(_ notification: Notification) {
        print("[Window] Did Miniaturizing.")
    }
    func windowWillStartLiveResize(_ notification: Notification) {
        print("[Window] Will start live resizing.")
    }
    func windowWillResize(_ notification: Notification) {
        print("[Window] Will resize.")
    }
    func windowWillResize(_ sender: NSWindow, to frameSize: NSSize) -> NSSize {
        print("[Window] Will resize to \(frameSize).");  return frameSize;
    }
    func windowDidResize(_ notification: Notification) {
        print("[Window] Did resize.");
    }
    func windowDidEndLiveResize(_ notification: Notification) {
        print("[Window] Did end live resize."); resizeFramebuffer()
    }
    func windowShouldZoom(_ window: NSWindow, toFrame newFrame: NSRect) -> Bool {
        print("[Window] Should zoom."); return true
    }
    func windowWillUseStandardFrame(_ window: NSWindow, defaultFrame newFrame: NSRect) -> NSRect {
        print("[Window] Will use standard frame."); return newFrame;
    }
    func windowDidUpdate(_ notification: Notification) {
        // print("[Window] Did update.")  // Basically called after every notification, keypress or mouse action [https://developer.apple.com/documentation/appkit/nswindow/1419641-didupdatenotification]
    }
    func windowDidBecomeKey(_ notification: Notification) {
        print("[Window] Did become key.")
    }
    func windowDidResignKey(_ notification: Notification) {
        print("[Window] Did resign key.")
    }
    func windowDidBecomeMain(_ notification: Notification) {
        print("[Window] Did becom main.")
    }
    func windowDidResignMain(_ notification: Notification) {
        print("[Window] Did resign main.")
    }
    func windowWillMove(_ notification: Notification) {
        print("[Window] Will move.")
    }
    func windowDidMove(_ notification: Notification) {
        print("[Window] Did move.")
    }
    func windowDidChangeScreen(_ notification: Notification) {
        print("[Window] Did change screens.")
    }
    func windowDidExpose(_ notification: Notification) {
        print("[Window] Did expose.")
    }
    func windowWillClose(_ notification: Notification) {
        print("[Window] Will close.")
    }
    func windowShouldClose(_ sender: NSWindow) -> Bool {
        print("[Window] Should close."); return true
    }
    func windowWillEnterFullScreen(_ notification: Notification) {
        print("[Window] Will enter fullscreen.")
    }
    func windowDidEnterFullScreen(_ notification: Notification) {
        print("[Window] Did enter full screen.")
    }
    func windowDidFailToEnterFullScreen(_ window: NSWindow) {
        print("[Window] Did fail to enter fullscreen.")
    }
    func windowWillExitFullScreen(_ notification: Notification) {
        print("[Window] Will exit fullscreen.")
    }
    func windowDidExitFullScreen(_ notification: Notification) {
        print("[Window] Did exit fullscreen.")
    }
    func windowDidFailToExitFullScreen(_ window: NSWindow) {
        print("[Window] Failed to exit fullscreen.")
    }
    
    
    // ---- Other ----
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
    
    func resizeFramebuffer() {
        // if let framebuffer = self.framebuffer {
            // framebuffer.pixels.deallocate()  // @TODO(ted): Leak somewhere...
        // }
        
        let width  = Int(self.view.window?.frame.size.width ?? 0)
        let height = Int(self.view.window?.frame.size.height ?? 0)
        
        self.framebuffer = Rust_CFramebuffer(
            width:  width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_Color>.allocate(capacity: width * height)
        )
        self.dirty = true
        self.updateFramebuffer()
    }
}



class GameView: NSView {
    var image: NSImage?

    override func draw(_ dirtyRect: NSRect) {
        self.image?.draw(in: dirtyRect, from: .zero, operation: .copy, fraction: 1.0)
    }
}
