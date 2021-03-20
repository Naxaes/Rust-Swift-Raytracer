//
//  Toolbar.swift
//  MacOSPlatform
//
//  Created by Ted Klein Bergman on 2021-03-19.
//

import Cocoa


class AppToolbar: NSToolbar, NSToolbarDelegate {
    override init(identifier: NSToolbar.Identifier) {
        super.init(identifier: identifier)
        self.delegate = self
    }
    
    func toolbar(
        _ toolbar: NSToolbar,
        itemForItemIdentifier itemIdentifier: NSToolbarItem.Identifier,
        willBeInsertedIntoToolbar flag: Bool
    ) -> NSToolbarItem? {
        return nil
        
    }
    func toolbarAllowedItemIdentifiers(_ toolbar: NSToolbar) -> [NSToolbarItem.Identifier] {
        return [.print, .showColors, .flexibleSpace,. space] // Whatever items you want to allow
    }
    func toolbarDefaultItemIdentifiers(_ toolbar: NSToolbar) -> [NSToolbarItem.Identifier] {
        return [.flexibleSpace, .showColors] // Whatever items you want as default
    }
}
