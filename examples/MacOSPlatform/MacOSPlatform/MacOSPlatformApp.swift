//
//  MacOSPlatformApp.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-17.
//

import SwiftUI

@main
struct MacOSPlatformApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
    
    init() {
        let result = rust_hello("world")
        let swift_result = String(cString: result!)
        rust_hello_free(UnsafeMutablePointer(mutating: result))
        print(swift_result)
    }
}
