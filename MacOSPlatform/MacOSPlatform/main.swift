//
//  main.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-19.
//

import Cocoa

let delegate = AppDelegate()
let menu     = AppMenu()

NSApplication.shared.delegate = delegate
NSApplication.shared.mainMenu = menu
_ = NSApplicationMain(CommandLine.argc, CommandLine.unsafeArgv)
