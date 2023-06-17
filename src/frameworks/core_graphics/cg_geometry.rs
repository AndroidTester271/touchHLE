/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CGGeometry.h` (`CGPoint`, `CGSize`, `CGRect`, etc)
//!
//! See also [crate::frameworks::uikit::ui_geometry].

use super::CGFloat;
use crate::abi::{impl_GuestRet_for_large_struct, GuestArg};
use crate::mem::SafeRead;

fn parse_tuple(s: &str) -> Result<(f32, f32), ()> {
    let (a, b) = s.split_once(", ").ok_or(())?;
    Ok((a.parse().map_err(|_| ())?, b.parse().map_err(|_| ())?))
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}
unsafe impl SafeRead for CGPoint {}
impl_GuestRet_for_large_struct!(CGPoint);
impl GuestArg for CGPoint {
    const REG_COUNT: usize = 2;

    fn from_regs(regs: &[u32]) -> Self {
        CGPoint {
            x: GuestArg::from_regs(&regs[0..1]),
            y: GuestArg::from_regs(&regs[1..2]),
        }
    }
    fn to_regs(self, regs: &mut [u32]) {
        self.x.to_regs(&mut regs[0..1]);
        self.y.to_regs(&mut regs[1..2]);
    }
}
impl std::str::FromStr for CGPoint {
    type Err = ();
    fn from_str(s: &str) -> Result<CGPoint, ()> {
        let s = s.strip_prefix('{').ok_or(())?.strip_suffix('}').ok_or(())?;
        let (x, y) = parse_tuple(s)?;
        Ok(CGPoint { x, y })
    }
}
impl std::fmt::Display for CGPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let &CGPoint { x, y } = self;
        write!(f, "{{{}, {}}}", x, y)
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}
unsafe impl SafeRead for CGSize {}
impl_GuestRet_for_large_struct!(CGSize);
impl GuestArg for CGSize {
    const REG_COUNT: usize = 2;

    fn from_regs(regs: &[u32]) -> Self {
        CGSize {
            width: GuestArg::from_regs(&regs[0..1]),
            height: GuestArg::from_regs(&regs[1..2]),
        }
    }
    fn to_regs(self, regs: &mut [u32]) {
        self.width.to_regs(&mut regs[0..1]);
        self.height.to_regs(&mut regs[1..2]);
    }
}
impl std::str::FromStr for CGSize {
    type Err = ();
    fn from_str(s: &str) -> Result<CGSize, ()> {
        let s = s.strip_prefix('{').ok_or(())?.strip_suffix('}').ok_or(())?;
        let (w, h) = parse_tuple(s)?;
        Ok(CGSize {
            width: w,
            height: h,
        })
    }
}
impl std::fmt::Display for CGSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let &CGSize { width, height } = self;
        write!(f, "{{{}, {}}}", width, height)
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C, packed)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
unsafe impl SafeRead for CGRect {}
impl_GuestRet_for_large_struct!(CGRect);
impl GuestArg for CGRect {
    const REG_COUNT: usize = 4;

    fn from_regs(regs: &[u32]) -> Self {
        CGRect {
            origin: GuestArg::from_regs(&regs[0..2]),
            size: GuestArg::from_regs(&regs[2..4]),
        }
    }
    fn to_regs(self, regs: &mut [u32]) {
        self.origin.to_regs(&mut regs[0..2]);
        self.size.to_regs(&mut regs[2..4]);
    }
}
impl std::str::FromStr for CGRect {
    type Err = ();
    fn from_str(s: &str) -> Result<CGRect, ()> {
        let s = s
            .strip_prefix("{{")
            .ok_or(())?
            .strip_suffix("}}")
            .ok_or(())?;
        let (a, b) = s.split_once("}, {").ok_or(())?;
        let (x, y) = parse_tuple(a)?;
        let (width, height) = parse_tuple(b)?;
        Ok(CGRect {
            origin: CGPoint { x, y },
            size: CGSize { width, height },
        })
    }
}
impl std::fmt::Display for CGRect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let &CGRect { origin, size } = self;
        write!(f, "{{{}, {}}}", origin, size)
    }
}
