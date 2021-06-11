use cocoa::appkit::NSPasteboardTypeString;
use cocoa::base::{id, nil, BOOL, YES};
use cocoa::foundation::{NSString, NSInteger};
use objc::{class, msg_send, sel, sel_impl};

#[derive(Debug, Clone, Default)]
pub struct Clipboard;

impl Clipboard {
    pub fn put_string(&mut self, s: impl AsRef<str>) {
        let s = s.as_ref();
         unsafe {
            let nsstring = NSString::alloc(nil).init_str(s);
            let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
            let _: NSInteger = msg_send![pasteboard, clearContents];
            let result: BOOL =
               msg_send![pasteboard, setString: nsstring forType: NSPasteboardTypeString];
            if result != YES {
               println!("failed to set clipboard");
            }
         }
    }

    pub fn get_string(&self) -> Option<String> {
        unsafe {
            let pasteboard: id = msg_send![class!(NSPasteboard), generalPasteboard];
            let contents: id = msg_send![pasteboard, stringForType: NSPasteboardTypeString];
            if contents.is_null() {
                None
            } else {
               let slice = std::slice::from_raw_parts(contents.UTF8String() as *const _, contents.len());
               let result = std::str::from_utf8_unchecked(slice);
               Some(result.to_string())
            }
        }
    }
}