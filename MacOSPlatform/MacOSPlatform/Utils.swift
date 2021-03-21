//
//  Utils.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-20.
//

import Cocoa


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
