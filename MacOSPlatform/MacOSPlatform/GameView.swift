//
//  GameView.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-18.
//

import Cocoa


@objc(MyApplication)
class App: NSApplication {
    // TODO: Do it properly. Or is it correct? It seems to be set to false on close.
    override var isRunning: Bool { get { return true } }
    
    override func run() {
        self.finishLaunching()
        repeat {
            
            let maybe_event = nextEvent(matching: .any, until: .distantFuture, inMode: .default, dequeue: true)
            if let event = maybe_event {
                self.sendEvent(event)
            }
            self.updateWindows()

        } while self.isRunning
    }
}


// NOTE: Handles the state of the application.
class AppDelegate: NSObject, NSApplicationDelegate {
    private var window: NSWindow?
    
    // ---- SETTINGS ----
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
    
    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        // return .terminateLater   // Proceed with termination later. NSApplication.shared.reply(toApplicationShouldTerminate: true)
        // return .terminateCancel  // Application should not be terminated
        return .terminateNow        // Proceed with termination.
    }
    
    
    // ---- CALLBACKS ----
    func applicationWillFinishLaunching(_ notification: Notification) {
        print("[Application] Did finish launching.")
    }
    func applicationDidFinishLaunching(_ notification: Notification) {
        print("[Application] Did finish launching.");
        self.window = GameWindow()
        self.window?.delegate = self.window as? NSWindowDelegate
    }
    
    func applicationWillHide(_ notification: Notification) {
        print("[Application] Will hide.")
    }
    func applicationDidHide(_ notification: Notification) {
        print("[Application] Did hide.")
    }
    
    func applicationWillUnhide(_ notification: Notification) {
        print("[Application] Will unhide.")
    }
    func applicationDidUnhide(_ notification: Notification) {
        print("[Application] Did unhide.")
    }
    
    func applicationWillUpdate(_ notification: Notification) {
        // print("[Application] Will update.")
    }
    func applicationDidUpdate(_ notification: Notification) {
        // print("[Application] Did update.")
    }
    
    func applicationWillBecomeActive(_ notification: Notification) {
        print("[Application] Will become active.")
    }
    func applicationDidBecomeActive(_ notification: Notification) {
        print("[Application] Did become active.")
    }
    
    func applicationWillResignActive(_ notification: Notification) {
        print("[Application] Will resign active.")
    }
    func applicationDidResignActive(_ notification: Notification) {
        print("[Application] Did resign active.")
    }
    
    func applicationDidChangeOcclusionState(_ notification: Notification) {
        // TODO: How do you do this properly?
        assert(NSApplication.shared.occlusionState.rawValue == 8194 || NSApplication.shared.occlusionState.rawValue == 8192)
        if (NSApplication.shared.occlusionState.rawValue == 8194) { print("[Application] Did change occlusion state (now visible).") }
        else { print("[Application] Did change occlusion state (now not visible).") }
    }
    
    func applicationDidChangeScreenParameters(_ notification: Notification) {
        print("[Application] Did change screen parameters.")
    }
    
    func applicationWillTerminate(_ notification: Notification) {
        print("[Application] Will terminate.")
    }
}


// NOTE: Handles a single window.
import Carbon
class GameWindow : NSWindow, NSWindowDelegate {
    var game: GameView  // TODO: Solve this terrible hack...
    var world: UnsafeMutablePointer<Rust_WorldHandle>
    var framebuffer: Rust_CFramebuffer
    
    var dirty: Bool = true
    
    
    override init(contentRect: NSRect, styleMask style: NSWindow.StyleMask, backing backingStoreType: NSWindow.BackingStoreType, defer flag: Bool) {
        let width  = Int(contentRect.width)
        let height = Int(contentRect.height)
        
        self.game = GameView()
        self.world = load_world(world_source())
        self.framebuffer = Rust_CFramebuffer(
            width:  width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_ColorU8>.allocate(capacity: width * height)
        )
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
        
        // self.toolbar = AppToolbar(identifier: .init("Default"))
        self.makeKeyAndOrderFront(nil)
        
        self.hasShadow = false
    
        // Make window fully transparent.
        self.isOpaque = false
        // self.alphaValue = 1.0  // NOTE: Lol! Didn't expect this. This affects the whole window though, even the GameView.
        self.backgroundColor = .clear
        
        self.becomeKey()
        self.becomeMain()
        
        self.contentView = self.game
        
        if let screen = NSScreen.main {
            let rect   = screen.frame
            let height = rect.size.height * 0.5
            let width  = rect.size.width  * 0.5
            
            self.setFrame(NSMakeRect(width / 2.0, height / 2.0, width, height), display: true)
            self.resizeAndUpdateFramebuffer()
        }
    }
    
    
    // Must be overridden for borderless style
    override var canBecomeKey:  Bool { return true }
    override var canBecomeMain: Bool { return true }
    // override var canBecomeVisibleWithoutLogin: Bool = true  //  TODO: WHAT IS THIS?
    override var firstResponder: NSResponder? { get { return self }}
    override var acceptsFirstResponder: Bool { get { return true } }
    
    

    override func nextEvent(
        matching mask: NSEvent.EventTypeMask,
        until expiration: Date?,
        inMode mode: RunLoop.Mode,
        dequeue deqFlag: Bool
    ) -> NSEvent? {
        return super.nextEvent(matching: mask, until: expiration, inMode: mode, dequeue: deqFlag)
    }
    
    override func nextEvent(matching mask: NSEvent.EventTypeMask) -> NSEvent? {
        return super.nextEvent(matching: mask)
    }
    
    
    // ---- VIEW EVENT CALLBACKS ----
    override func keyDown(with event: NSEvent) {
        if event.characters == "a" {
            self.world.pointee.camera = move_camera_position(self.world.pointee.camera, -0.1, 0.0, 0.0);
        } else if event.characters == "d" {
            self.world.pointee.camera = move_camera_position(self.world.pointee.camera,  0.1, 0.0, 0.0);
        } else if event.characters == "w" {
            self.world.pointee.camera = move_camera_position(self.world.pointee.camera,  0.0, 0.0, -0.1);
        } else if event.characters == "s" {
            self.world.pointee.camera = move_camera_position(self.world.pointee.camera,  0.0, 0.0,  0.1);
        } else if event.characters == " " {
            self.world.pointee.camera = move_camera_position(self.world.pointee.camera,  0.0, 0.1,  0.0);
        }
        
        self.resizeAndUpdateFramebuffer()
    }
    override func keyUp(with event: NSEvent) {}
    override func flagsChanged(with event: NSEvent) {
        if event.keyCode == 59 /* Left shift */ {
            self.world.pointee.camera = move_camera_position(self.world.pointee.camera,  0.0, -0.1,  0.0);
        }
        self.resizeAndUpdateFramebuffer()
    }


    override func mouseDown(with event: NSEvent) {}
    override func rightMouseDown(with event: NSEvent) {}
    override func otherMouseDown(with event: NSEvent) {}
    
    override func mouseUp(with event: NSEvent) {}
    override func rightMouseUp(with event: NSEvent) {}
    override func otherMouseUp(with event: NSEvent) {}
    
    override func mouseMoved(with event: NSEvent) {}
    override func mouseDragged(with event: NSEvent) {}
    override func rightMouseDragged(with event: NSEvent) {}
    override func otherMouseDragged(with event: NSEvent) {}
    
    override func scrollWheel(with event: NSEvent) {}
    
    override func mouseEntered(with event: NSEvent) {}
    override func mouseExited(with event: NSEvent) {}
    

    // ---- WINDOW CALLBACKS ----
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
        /*print("[Window] Will resize to \(frameSize)."); */ return frameSize;
    }
    func windowDidResize(_ notification: Notification) {
        print("[Window] Did resize.");
    }
    func windowDidEndLiveResize(_ notification: Notification) {
        print("[Window] Did end live resize."); self.resizeFramebufferIfNeeded()
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
    func updateFramebufferIfDirty() {
        if !self.dirty {
            return
        }

        self.framebuffer = render(self.framebuffer, self.world)
        self.game.image = NSImage(framebuffer: self.framebuffer) // ?.roundCorners(withRadius: 32)
        DispatchQueue.main.async {
            self.game.setNeedsDisplay(self.game.visibleRect)
        }
        self.dirty = false
    }
    
    func resizeFramebufferIfNeeded() {
        if self.framebuffer.width < Int(self.frame.size.width) || self.framebuffer.height < Int(self.frame.size.width) {
            self.resizeAndUpdateFramebuffer()
        }
    }
    
    func resizeAndUpdateFramebuffer() {
        // if let framebuffer = self.framebuffer {
            // framebuffer.pixels.deallocate()  // @TODO(ted): Leak somewhere...
        // }
        
        let width  = Int(self.frame.size.width)
        let height = Int(self.frame.size.height)
        
        self.framebuffer = Rust_CFramebuffer(
            width:  width,
            height: height,
            pixels: UnsafeMutablePointer<Rust_ColorU8>.allocate(capacity: width * height)
        )
        self.dirty = true
        self.updateFramebufferIfDirty()
    }
}



class GameView: NSView {
    var image: NSImage?
    
    override func draw(_ dirtyRect: NSRect) {
        self.image?.draw(in: dirtyRect, from: .zero, operation: .copy, fraction: 1.0)
    }
}
