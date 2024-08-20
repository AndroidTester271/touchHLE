/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITextField`.

use crate::frameworks::foundation::{ns_string, NSInteger, NSRange};
use crate::impl_HostObject_with_superclass;
use crate::objc::{id, msg, objc_classes, nil, ClassExports, NSZonePtr};

type UIKeyboardAppearance = NSInteger;
type UIKeyboardType = NSInteger;
type UIReturnKeyType = NSInteger;
type UITextAutocapitalizationType = NSInteger;
type UITextAutocorrectionType = NSInteger;

struct UITextFieldHostObject {
    superclass: super::UIControlHostObject,
    delegate: id
}
impl_HostObject_with_superclass!(UITextFieldHostObject);
impl Default for UITextFieldHostObject {
    fn default() -> Self {
        UITextFieldHostObject {
            superclass: Default::default(),
            delegate: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITextField: UIControl

// TODO: rendering
// TODO: more properties

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITextFieldHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)text {
    // This should be `nil` by default, but Wolf3d crashes otherwise
    ns_string::get_static_str(env, "test")
}

- (())setText:(id)_text { // NSString*
    log!("setText:");
    // TODO
}
- (())setTextColor:(id)_color { // UIColor*
    log!("setTextColor:");
    // TODO: implement this once views are actually rendered
}

- (())setClearsOnBeginEditing:(bool)_clear {
    log!("setClearsOnBeginEditing:");
    // TODO
}

- (())setClearButtonMode:(NSInteger)_mode {
    log!("setClearButtonMode:");
    // TODO
}

// weak/non-retaining
- (())setDelegate:(id)delegate { // something implementing UITextFieldDelegate
    log!("setDelegate:");
    // TODO
    let host_object = env.objc.borrow_mut::<UITextFieldHostObject>(this);
    host_object.delegate = delegate;
}
- (id)delegate {
    env.objc.borrow::<UITextFieldHostObject>(this).delegate
}

// UITextInputTraits implementation
- (())setAutocapitalizationType:(UITextAutocapitalizationType)_type {
    log!("setAutocapitalizationType:");
    // TODO
}
- (())setAutocorrectionType:(UITextAutocorrectionType)_type {
    log!("setAutocorrectionType:");
    // TODO
}
- (())setReturnKeyType:(UIReturnKeyType)_type {
    log!("setReturnKeyType:");
    // TODO
}
- (())setKeyboardAppearance:(UIKeyboardAppearance)_appearance {
    log!("setKeyboardAppearance:");
    // TODO
}
- (())setKeyboardType:(UIKeyboardType)_type {
    log!("setKeyboardType:");
    // TODO
}
- (())setBorderStyle:(NSInteger)_style {
    log!("setBorderStyle:");
    // TODO
}
- (())setFont:(id)new_font { // UIFont*
    log!("setFont:");
}

- (bool)becomeFirstResponder {
    log!("becomeFirstResponder");
    // TODO
    let delegate: id = env.objc.borrow::<UITextFieldHostObject>(this).delegate;
    assert!(delegate != nil);
    () = msg![env; delegate textFieldDidBeginEditing:this];
    // (BOOL)textField:(UITextField *)textField
    // shouldChangeCharactersInRange:(NSRange)range
    // replacementString:(NSString *)string;
    let txt = ns_string::get_static_str(env, "t");
    let range = NSRange { location: 0, length: 1 };
    () = msg![env; delegate textField:this shouldChangeCharactersInRange:range replacementString:txt];
    let txt = ns_string::get_static_str(env, "e");
    let range = NSRange { location: 1, length: 1 };
    () = msg![env; delegate textField:this shouldChangeCharactersInRange:range replacementString:txt];
    let txt = ns_string::get_static_str(env, "s");
    let range = NSRange { location: 2, length: 1 };
    () = msg![env; delegate textField:this shouldChangeCharactersInRange:range replacementString:txt];
    let txt = ns_string::get_static_str(env, "t");
    let range = NSRange { location: 3, length: 1 };
    () = msg![env; delegate textField:this shouldChangeCharactersInRange:range replacementString:txt];
    () = msg![env; delegate textFieldShouldReturn:this];
    () = msg![env; delegate textFieldDidEndEditing:this];
    false
}

@end

};
