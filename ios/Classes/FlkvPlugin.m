#import "FlkvPlugin.h"
#if __has_include(<flkv/flkv-Swift.h>)
#import <flkv/flkv-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "flkv-Swift.h"
#endif

@implementation FlkvPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftFlkvPlugin registerWithRegistrar:registrar];
}
@end
