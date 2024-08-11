/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UIImagePickerController`

use crate::frameworks::foundation::NSInteger;
use crate::objc::{objc_classes, ClassExports};
use crate::objc::{id, SEL, nil};

type UIImagePickerControllerSourceType = NSInteger;

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// TODO: should extend UINavigationController, which extends
//       UIViewController.
@implementation UIImagePickerController: UIViewController

+ (bool)isSourceTypeAvailable:(UIImagePickerControllerSourceType)_type {
    // For now, simply claim no sources are available.
    // TODO: support some sources.
    false
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())setSourceType:(bool)source {
    log!("TODO: setSourceType:{}", source);
}
@end

@implementation UIBarButtonItem: NSObject
- (id)initWithTitle:(id)title style:(NSInteger)style target:(id)target action:(SEL)action {
    nil
}
- (id)initWithBarButtonSystemItem:(NSInteger)item target:(id)target action:(SEL)action {
    nil
}
- (id)initWithCoder:(id)coder {
    nil
}
- (id)initWithCustomView:(id)view {
    nil
}
@end

};
