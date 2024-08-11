use crate::objc::{id, msg, objc_classes, ClassExports};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation iSimulate: UIView
+ (bool)disableTouchesOverlay:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

@end

};
