hs.hotkey.bind({"cmd","ctrl"}, "V", function() hs.eventtap.keyStrokes(hs.pasteboard.getContents()) end)
