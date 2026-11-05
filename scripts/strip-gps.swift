// remove gps location data from images, in place, without re-encoding them.
// phone photos carry where they were taken (often home), and sips keeps it.
//
// usage: swift scripts/strip-gps.swift <image>...
import Foundation
import ImageIO

var failed = false
for path in CommandLine.arguments.dropFirst() {
  let url = URL(fileURLWithPath: path)
  guard let src = CGImageSourceCreateWithURL(url as CFURL, nil),
        let type = CGImageSourceGetType(src),
        let original = CGImageSourceCopyMetadataAtIndex(src, 0, nil),
        let metadata = CGImageMetadataCreateMutableCopy(original) else {
    print("can't read \(path)"); failed = true; continue
  }
  // drop every gps tag and keep the rest (orientation in particular, or photos
  // taken sideways come out rotated). kCGImageMetadataShouldExcludeGPS would
  // be simpler, but it can't be combined with keeping the orientation
  var gps: [String] = []
  CGImageMetadataEnumerateTagsUsingBlock(original, nil, [kCGImageMetadataEnumerateRecursively: true] as CFDictionary) { path, tag in
    let name = CGImageMetadataTagCopyName(tag) as String? ?? ""
    let prefix = CGImageMetadataTagCopyPrefix(tag) as String? ?? ""
    if prefix == "exif" && name.hasPrefix("GPS") { gps.append(path as String) }
    return true
  }
  for tag in gps where !tag.contains("/") {
    CGImageMetadataRemoveTagWithPath(metadata, nil, tag as CFString)
  }

  let data = NSMutableData()
  guard let dest = CGImageDestinationCreateWithData(data, type, 1, nil) else { failed = true; continue }
  let opts = [kCGImageDestinationMetadata: metadata, kCGImageDestinationMergeMetadata: false] as CFDictionary
  var err: Unmanaged<CFError>?
  guard CGImageDestinationCopyImageSource(dest, src, opts, &err) else {
    print("can't strip \(path): \(String(describing: err?.takeRetainedValue()))"); failed = true; continue
  }
  data.write(to: url, atomically: true)
}
exit(failed ? 1 : 0)
