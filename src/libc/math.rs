/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `math.h`

use std::num::FpCategory;
use crate::dyld::{export_c_func, FunctionExports};
use crate::Environment;
use crate::mem::{MutPtr, MutVoidPtr};

// The sections in this file are organized to match the C standard.

// FIXME: Many functions in this file should theoretically set errno or affect
//        the floating-point environment. We're hoping apps won't rely on that.

// Trigonometric functions

// TODO: These should also have `long double` variants, which can probably just
// alias the `double` ones.

fn sin(_env: &mut Environment, arg: f64) -> f64 {
    arg.sin()
}
fn sinf(_env: &mut Environment, arg: f32) -> f32 {
    arg.sin()
}
fn cos(_env: &mut Environment, arg: f64) -> f64 {
    arg.cos()
}
fn cosf(_env: &mut Environment, arg: f32) -> f32 {
    arg.cos()
}
fn tan(_env: &mut Environment, arg: f64) -> f64 {
    arg.tan()
}
fn tanf(_env: &mut Environment, arg: f32) -> f32 {
    arg.tan()
}

fn asin(_env: &mut Environment, arg: f64) -> f64 {
    arg.asin()
}
fn asinf(_env: &mut Environment, arg: f32) -> f32 {
    arg.asin()
}
fn acos(_env: &mut Environment, arg: f64) -> f64 {
    arg.acos()
}
fn acosf(_env: &mut Environment, arg: f32) -> f32 {
    arg.acos()
}
fn atan(_env: &mut Environment, arg: f64) -> f64 {
    arg.atan()
}
fn atanf(_env: &mut Environment, arg: f32) -> f32 {
    arg.atan()
}

fn atan2f(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.atan2(arg2)
}
fn atan2(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.atan2(arg2)
}

// Hyperbolic functions

fn sinh(_env: &mut Environment, arg: f64) -> f64 {
    arg.sinh()
}
fn sinhf(_env: &mut Environment, arg: f32) -> f32 {
    arg.sinh()
}
fn cosh(_env: &mut Environment, arg: f64) -> f64 {
    arg.cosh()
}
fn coshf(_env: &mut Environment, arg: f32) -> f32 {
    arg.cosh()
}
fn tanh(_env: &mut Environment, arg: f64) -> f64 {
    arg.tanh()
}
fn tanhf(_env: &mut Environment, arg: f32) -> f32 {
    arg.tanh()
}

fn asinh(_env: &mut Environment, arg: f64) -> f64 {
    arg.asinh()
}
fn asinhf(_env: &mut Environment, arg: f32) -> f32 {
    arg.asinh()
}
fn acosh(_env: &mut Environment, arg: f64) -> f64 {
    arg.acosh()
}
fn acoshf(_env: &mut Environment, arg: f32) -> f32 {
    arg.acosh()
}
fn atanh(_env: &mut Environment, arg: f64) -> f64 {
    arg.atanh()
}
fn atanhf(_env: &mut Environment, arg: f32) -> f32 {
    arg.atanh()
}

// Exponential and logarithmic functions
// TODO: implement the rest
fn log(_env: &mut Environment, arg: f64) -> f64 {
    arg.ln()
}
fn logf(_env: &mut Environment, arg: f32) -> f32 {
    arg.ln()
}
fn log1p(_env: &mut Environment, arg: f64) -> f64 {
    arg.ln_1p()
}
fn log1pf(_env: &mut Environment, arg: f32) -> f32 {
    arg.ln_1p()
}
fn log2(_env: &mut Environment, arg: f64) -> f64 {
    arg.log2()
}
fn log2f(_env: &mut Environment, arg: f32) -> f32 {
    arg.log2()
}
fn log10(_env: &mut Environment, arg: f64) -> f64 {
    arg.log10()
}
fn log10f(_env: &mut Environment, arg: f32) -> f32 {
    arg.log10()
}
fn ldexp(_env: &mut Environment, arg: f64) -> f64 {
    arg.exp()
}
fn exp(_env: &mut Environment, arg: f64) -> f64 {
    arg.exp()
}
fn expf(_env: &mut Environment, arg: f32) -> f32 {
    arg.exp()
}
fn expm1(_env: &mut Environment, arg: f64) -> f64 {
    arg.exp_m1()
}
fn expm1f(_env: &mut Environment, arg: f32) -> f32 {
    arg.exp_m1()
}
fn exp2(_env: &mut Environment, arg: f64) -> f64 {
    arg.exp2()
}
fn exp2f(_env: &mut Environment, arg: f32) -> f32 {
    arg.exp2()
}

// Power functions
// TODO: implement the rest
fn pow(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.powf(arg2)
}
fn powf(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.powf(arg2)
}
fn sqrt(_env: &mut Environment, arg: f64) -> f64 {
    arg.sqrt()
}
fn sqrtf(_env: &mut Environment, arg: f32) -> f32 {
    arg.sqrt()
}

// Nearest integer functions
// TODO: implement the rest
fn ceil(_env: &mut Environment, arg: f64) -> f64 {
    arg.ceil()
}
fn ceilf(_env: &mut Environment, arg: f32) -> f32 {
    arg.ceil()
}
fn floor(_env: &mut Environment, arg: f64) -> f64 {
    arg.floor()
}
fn floorf(_env: &mut Environment, arg: f32) -> f32 {
    arg.floor()
}
fn round(_env: &mut Environment, arg: f64) -> f64 {
    arg.round()
}
fn roundf(_env: &mut Environment, arg: f32) -> f32 {
    arg.round()
}
fn trunc(_env: &mut Environment, arg: f64) -> f64 {
    arg.trunc()
}
fn truncf(_env: &mut Environment, arg: f32) -> f32 {
    arg.trunc()
}
// float
//      modff(float value, float *iptr)
fn modff(env: &mut Environment, val: f32, iptr: MutPtr<f32>) -> f32 {
    let ivalue = truncf(env, val);
    env.mem.write(iptr, ivalue);
    val - ivalue
}

// Remainder functions
// TODO: implement the rest
fn fmod(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1 % arg2
}
fn fmodf(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1 % arg2
}

// Maximum, minimum and positive difference functions
// TODO: implement fdim
fn bind(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn clock_get_time(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn connect(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn div(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}

fn fmax(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.max(arg2)
}
fn fmaxf(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.max(arg2)
}
fn fmin(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn fminf(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn fork(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn gethostname(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn host_get_clock_service(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioQueueAllocateBufferWithPacketDescriptions(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioFileOpenWithCallbacks(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioServicesAddSystemSoundCompletion(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioServicesDisposeSystemSoundID(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn AudioSessionGetPropertySize(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CACurrentMediaTime(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFAbsoluteTimeGetDifferenceAsGregorianUnits(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFBundleCopyResourceURLForLocalization(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFBundleGetInfoDictionary(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFLocaleCopyCurrent(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFRunLoopRun(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFStringGetSystemEncoding(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CFURLGetString(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGColorGetConstantColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetAllowsAntialiasing(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGContextSetRGBStrokeColor(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGDataProviderCreateDirect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreate(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreateWithJPEGDataProvider(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGImageCreateWithImageInRect(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGLayerCreateWithContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn CGLayerGetContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn MFMailComposeErrorDomain(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn NSFullUserName(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn SCNetworkReachabilitySetCallback(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn SCNetworkReachabilityScheduleWithRunLoop(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIApplicationDidReceiveMemoryWarningNotification(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIGraphicsEndImageContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn UIGraphicsGetImageFromCurrentImageContext(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glDrawTexiOES(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glBindFramebuffer(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glBlendEquationOES(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCheckFramebufferStatus(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glFramebufferRenderbuffer(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glFramebufferTexture2D(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGenFramebuffers(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glRenderbufferStorage(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCreateShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glShaderSource(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glSampleCoverage(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCompileShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetShaderiv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glCreateProgram(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glAttachShader(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glLinkProgram(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetProgramiv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetAttribLocation(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetUniformLocation(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glGetTexParameteriv(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glReadPixels(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn glPointSizePointerOES(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn abort(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn abs(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn asctime(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn dladdr(_env: &mut Environment, arg1: f32, arg2: f32) -> f32 {
    arg1.min(arg2)
}
fn gzopen(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn gzread(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn gzclose(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn gzeof(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inflate(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inflateInit_(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inflateInit2_(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inflateEnd(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn mprotect(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn pipe(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn ptrace(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn rename(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn setsockopt(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn strcasestr(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn getsockname(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn socket(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn ioctl(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inet_addr(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inet_ntoa(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn inet_pton(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn listen(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn kqueue(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn uncompress(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn wcstok(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn wcstod(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn _exit(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn __sprintf_chk(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn __maskrune(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn rintf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sbrk(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn shm_open(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_open(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_errcode(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_errmsg(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_prepare_v2(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_step(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_finalize(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_mprintf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_close(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_reset(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_int(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sqlite3_bind_double(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn sranddev(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn strpbrk(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn putc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn getc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn ungetc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn fgetwc(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn fscanf(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn writev(_env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    arg1.min(arg2)
}
fn hypot(env: &mut Environment, arg1: f64, arg2: f64) -> f64 {
    sqrt(env, arg1*arg1 + arg2*arg2)
}

fn lrint(_env: &mut Environment, arg1: f64) -> i64 {
    arg1.max(i64::MIN as f64).min(i64::MAX as f64).round() as i64
}

fn lrintf(_env: &mut Environment, arg1: f32) -> i32 {
    arg1.max(i32::MIN as f32).min(i32::MAX as f32).round() as i32
}

type GuestFPCategory = i32;
const FP_NAN: GuestFPCategory = 1;
const FP_INFINITE: GuestFPCategory = 1;
const FP_ZERO: GuestFPCategory = 3;
const FP_NORMAL: GuestFPCategory = 4;
const FP_SUBNORMAL: GuestFPCategory = 4;

fn __fpclassifyf(_env: &mut Environment, arg: f32) -> GuestFPCategory {
    match arg.classify() {
        FpCategory::Nan => FP_NAN,
        FpCategory::Infinite => FP_INFINITE,
        FpCategory::Zero => FP_ZERO,
        FpCategory::Normal => FP_NORMAL,
        FpCategory::Subnormal => FP_SUBNORMAL
    }
}

// int32_t
//      OSAtomicAdd32Barrier(int32_t theAmount, volatile int32_t *theValue)
fn OSAtomicAdd32Barrier(
    env: &mut Environment, the_amount: i32, the_value: MutPtr<i32>
) -> i32 {
    let curr = env.mem.read(the_value);
    let new = curr + the_amount;
    env.mem.write(the_value, new);
    new
}

fn OSAtomicCompareAndSwap32(
    env: &mut Environment, old_value: i32, new_value: i32, the_value: MutPtr<i32>
) -> bool {
    OSAtomicCompareAndSwap32Barrier(env, old_value, new_value, the_value)
}

fn OSAtomicCompareAndSwapIntBarrier(
    env: &mut Environment, old_value: i32, new_value: i32, the_value: MutPtr<i32>
) -> bool {
    if old_value == env.mem.read(the_value) {
        env.mem.write(the_value, new_value);
        true
    } else {
        false
    }
}

fn OSAtomicCompareAndSwap32Barrier(
    env: &mut Environment, old_value: i32, new_value: i32, the_value: MutPtr<i32>
) -> bool {
    if old_value == env.mem.read(the_value) {
        env.mem.write(the_value, new_value);
        true
    } else {
        false
    }
}

// bool
//      OSAtomicCompareAndSwapPtr(void* oldValue, void* newValue, void* volatile *theValue);
fn OSAtomicCompareAndSwapPtr(
    env: &mut Environment, old_value: MutVoidPtr, new_value: MutVoidPtr, the_value: MutPtr<MutVoidPtr>
) -> bool {
    if old_value == env.mem.read(the_value) {
        env.mem.write(the_value, new_value);
        true
    } else {
        false
    }
}

// int32_t	OSAtomicAdd32( int32_t __theAmount, volatile int32_t *__theValue );
fn OSAtomicAdd32(env: &mut Environment, amount: i32, value_ptr: MutPtr<i32>) -> i32 {
    let value = env.mem.read(value_ptr);
    let new_value = value + amount;
    env.mem.write(value_ptr, new_value);
    new_value
}

type OSSpinLock = i32;

// void    OSSpinLockLock( volatile OSSpinLock *__lock );
fn OSSpinLockLock(env: &mut Environment, lock: MutPtr<OSSpinLock>) {
    let curr = env.mem.read(lock);
    assert_eq!(curr, 0);
    env.mem.write(lock, 1);
}

fn OSSpinLockUnlock(env: &mut Environment, lock: MutPtr<OSSpinLock>) {
    let curr = env.mem.read(lock);
    assert_eq!(curr, 1);
    env.mem.write(lock, 0);
}

fn OSMemoryBarrier(env: &mut Environment) {

}

fn fesetround(_env: &mut Environment, round: i32) {
    // TODO
}

pub const FUNCTIONS: FunctionExports = &[
    // Trigonometric functions
    export_c_func!(sin(_)),
    export_c_func!(sinf(_)),
    export_c_func!(cos(_)),
    export_c_func!(cosf(_)),
    export_c_func!(tan(_)),
    export_c_func!(tanf(_)),
    export_c_func!(asin(_)),
    export_c_func!(asinf(_)),
    export_c_func!(acos(_)),
    export_c_func!(acosf(_)),
    export_c_func!(atan(_)),
    export_c_func!(atanf(_)),
    export_c_func!(atan2(_, _)),
    export_c_func!(atan2f(_, _)),
    // Hyperbolic functions
    export_c_func!(sinh(_)),
    export_c_func!(sinhf(_)),
    export_c_func!(cosh(_)),
    export_c_func!(coshf(_)),
    export_c_func!(tanh(_)),
    export_c_func!(tanhf(_)),
    export_c_func!(asinh(_)),
    export_c_func!(asinhf(_)),
    export_c_func!(acosh(_)),
    export_c_func!(acoshf(_)),
    export_c_func!(atanh(_)),
    export_c_func!(atanhf(_)),
    // Exponential and logarithmic functions
    export_c_func!(log(_)),
    export_c_func!(logf(_)),
    export_c_func!(log1p(_)),
    export_c_func!(log1pf(_)),
    export_c_func!(log2(_)),
    export_c_func!(log2f(_)),
    export_c_func!(log10(_)),
    export_c_func!(log10f(_)),
    export_c_func!(ldexp(_)),
    export_c_func!(exp(_)),
    export_c_func!(expf(_)),
    export_c_func!(expm1(_)),
    export_c_func!(expm1f(_)),
    export_c_func!(exp2(_)),
    export_c_func!(exp2f(_)),
    // Power functions
    export_c_func!(pow(_, _)),
    export_c_func!(powf(_, _)),
    export_c_func!(sqrt(_)),
    export_c_func!(sqrtf(_)),
    // Nearest integer functions
    export_c_func!(ceil(_)),
    export_c_func!(ceilf(_)),
    export_c_func!(floor(_)),
    export_c_func!(floorf(_)),
    export_c_func!(round(_)),
    export_c_func!(roundf(_)),
    export_c_func!(trunc(_)),
    export_c_func!(truncf(_)),
    export_c_func!(modff(_, _)),
    // Remainder functions
    export_c_func!(fmod(_, _)),
    export_c_func!(fmodf(_, _)),
    // Maximum, minimum and positive difference functions
    export_c_func!(bind(_, _)),
    export_c_func!(clock_get_time(_, _)),
    export_c_func!(connect(_, _)),
    export_c_func!(div(_, _)),
    export_c_func!(fmax(_, _)),
    export_c_func!(fmaxf(_, _)),
    export_c_func!(fmin(_, _)),
    export_c_func!(fminf(_, _)),
    export_c_func!(fork(_, _)),
    export_c_func!(gethostname(_, _)),
    export_c_func!(host_get_clock_service(_, _)),
    export_c_func!(AudioQueueAllocateBufferWithPacketDescriptions(_, _)),
    export_c_func!(AudioFileOpenWithCallbacks(_, _)),
    export_c_func!(AudioServicesAddSystemSoundCompletion(_, _)),
    export_c_func!(AudioServicesDisposeSystemSoundID(_, _)),
    export_c_func!(AudioSessionGetPropertySize(_, _)),
    export_c_func!(CACurrentMediaTime(_, _)),
    export_c_func!(CFAbsoluteTimeGetDifferenceAsGregorianUnits(_, _)),
    export_c_func!(CFBundleCopyResourceURLForLocalization(_, _)),
    export_c_func!(CFBundleGetInfoDictionary(_, _)),
    export_c_func!(CFLocaleCopyCurrent(_, _)),
    export_c_func!(CFRunLoopRun(_, _)),
    export_c_func!(CFStringGetSystemEncoding(_, _)),
    export_c_func!(CFURLCreateStringByReplacingPercentEscapesUsingEncoding(_, _)),
    export_c_func!(CFURLGetString(_, _)),
    export_c_func!(CGColorGetConstantColor(_, _)),
    export_c_func!(CGContextSetAllowsAntialiasing(_, _)),
    export_c_func!(CGContextSetRGBStrokeColor(_, _)),
    export_c_func!(CGDataProviderCreateDirect(_, _)),
    export_c_func!(CGImageCreate(_, _)),
    export_c_func!(CGImageCreateWithJPEGDataProvider(_, _)),
    export_c_func!(CGImageCreateWithImageInRect(_, _)),
    export_c_func!(CGLayerCreateWithContext(_, _)),
    export_c_func!(CGLayerGetContext(_, _)),
    export_c_func!(MFMailComposeErrorDomain(_, _)),
    export_c_func!(NSFullUserName(_, _)),
    export_c_func!(SCNetworkReachabilitySetCallback(_, _)),
    export_c_func!(SCNetworkReachabilityScheduleWithRunLoop(_, _)),
    export_c_func!(UIApplicationDidReceiveMemoryWarningNotification(_, _)),
    export_c_func!(UIGraphicsEndImageContext(_, _)),
    export_c_func!(UIGraphicsGetImageFromCurrentImageContext(_, _)),
    export_c_func!(glDrawTexiOES(_, _)),
    export_c_func!(glBindFramebuffer(_, _)),
    export_c_func!(glBlendEquationOES(_, _)),
    export_c_func!(glCheckFramebufferStatus(_, _)),
    export_c_func!(glFramebufferRenderbuffer(_, _)),
    export_c_func!(glFramebufferTexture2D(_, _)),
    export_c_func!(glGenFramebuffers(_, _)),
    export_c_func!(glRenderbufferStorage(_, _)),
    export_c_func!(glCreateShader(_, _)),
    export_c_func!(glShaderSource(_, _)),
    export_c_func!(glSampleCoverage(_, _)),
    export_c_func!(glCompileShader(_, _)),
    export_c_func!(glGetShaderiv(_, _)),
    export_c_func!(glCreateProgram(_, _)),
    export_c_func!(glAttachShader(_, _)),
    export_c_func!(glLinkProgram(_, _)),
    export_c_func!(glGetProgramiv(_, _)),
    export_c_func!(glGetAttribLocation(_, _)),
    export_c_func!(glGetUniformLocation(_, _)),
    export_c_func!(glGetTexParameteriv(_, _)),
    export_c_func!(glReadPixels(_, _)),
    export_c_func!(glPointSizePointerOES(_, _)),
    export_c_func!(abort(_, _)),
    export_c_func!(abs(_, _)),
    export_c_func!(asctime(_, _)),
    export_c_func!(dladdr(_, _)),
    export_c_func!(gzopen(_, _)),
    export_c_func!(gzread(_, _)),
    export_c_func!(gzclose(_, _)),
    export_c_func!(gzeof(_, _)),
    export_c_func!(inflate(_, _)),
    export_c_func!(inflateInit_(_, _)),
    export_c_func!(inflateInit2_(_, _)),
    export_c_func!(inflateEnd(_, _)),
    export_c_func!(mprotect(_, _)),
    export_c_func!(pipe(_, _)),
    export_c_func!(ptrace(_, _)),
    export_c_func!(rename(_, _)),
    export_c_func!(setsockopt(_, _)),
    export_c_func!(strcasestr(_, _)),
    export_c_func!(getsockname(_, _)),
    export_c_func!(socket(_, _)),
    export_c_func!(ioctl(_, _)),
    export_c_func!(inet_addr(_, _)),
    export_c_func!(inet_ntoa(_, _)),
    export_c_func!(inet_pton(_, _)),
    export_c_func!(listen(_, _)),
    export_c_func!(kqueue(_, _)),
    export_c_func!(uncompress(_, _)),
    export_c_func!(wcstok(_, _)),
    export_c_func!(wcstod(_, _)),
    export_c_func!(_exit(_, _)),
    export_c_func!(__sprintf_chk(_, _)),
    export_c_func!(__maskrune(_, _)),
    export_c_func!(rintf(_, _)),
    export_c_func!(sbrk(_, _)),
    export_c_func!(shm_open(_, _)),
    export_c_func!(sqlite3_open(_, _)),
    export_c_func!(sqlite3_errcode(_, _)),
    export_c_func!(sqlite3_errmsg(_, _)),
    export_c_func!(sqlite3_prepare_v2(_, _)),
    export_c_func!(sqlite3_step(_, _)),
    export_c_func!(sqlite3_finalize(_, _)),
    export_c_func!(sqlite3_mprintf(_, _)),
    export_c_func!(sqlite3_close(_, _)),
    export_c_func!(sqlite3_reset(_, _)),
    export_c_func!(sqlite3_bind_int(_, _)),
    export_c_func!(sqlite3_bind_double(_, _)),
    export_c_func!(sranddev(_, _)),
    export_c_func!(strpbrk(_, _)),
    export_c_func!(putc(_, _)),
    export_c_func!(getc(_, _)),
    export_c_func!(ungetc(_, _)),
    export_c_func!(fgetwc(_, _)),
    export_c_func!(fscanf(_, _)),
    export_c_func!(writev(_, _)),
    export_c_func!(hypot(_, _)),
    export_c_func!(lrint(_)),
    export_c_func!(lrintf(_)),
    export_c_func!(__fpclassifyf(_)),
    export_c_func!(fesetround(_)),
    // Atomic ops (libkern)
    export_c_func!(OSAtomicCompareAndSwapIntBarrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwap32(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwap32Barrier(_, _, _)),
    export_c_func!(OSAtomicCompareAndSwapPtr(_, _, _)),
    export_c_func!(OSAtomicAdd32Barrier(_, _)),
    export_c_func!(OSAtomicAdd32(_, _)),
    export_c_func!(OSSpinLockLock(_)),
    export_c_func!(OSSpinLockUnlock(_)),
    export_c_func!(OSMemoryBarrier()),
];
