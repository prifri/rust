#![feature(link_llvm_intrinsics)]
#![allow(non_camel_case_types)]
#![cfg(not(windows))]

use libc:: {
    SIGUSR1, SIGALRM, SIGHUP, SIGQUIT, SIGTERM
};
use std::mem;

/*
 * prifri, 2022.11.04:
 * - 32bit 시스템에서는 4 * 8, 64bit 에서는 8 *8로 될것이다.
 */
const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 512;
/*
 * prifri, 2022.11.04:
 * - 32bit에서는 32byte, 64bit에서는 64byte
 * [i8; 32] or [i8; 64]
 */
type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C" {
/*
 * prifri, 2022.11.04:
 * - link_name을 적용하면 segmentaion fault. 지운다. 이유는 모른다.
 *   참고 : https://github.com/daku10/rust-in-actions-study/blob/09bcafdc64882d30a1e720b40c1e83dcad02763f/chapter-12/sjlj/src/main.rs
 */
    //#[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(_: *mut i8) -> i32;

    //#[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(_: *mut i8) -> ();
}

#[inline]
/*
 * prifri, 2022.11.04:
 * - &[0; JMP_BUF_WIDTH] -> *const i8
 *   읽기 전용 참조에서 읽기 전용포인터로 변경.
 *
 * - *const i8 -> *mut i8
 *   readonly에서 read /write 로 변경
 *
 * - &[0; JMP_BUF_WIDTH] -> *mut i8이 안되는 이유
 *   읽기 전용 참조 -> 읽기 전용 pointer 변경은 안전하지만
 *   읽기 전용 참ㅈ -> read/wrtie pointer변경은 rust가 안전하지 않다고
 *   판단한다.
 *   중간에 읽기전용참조 -> 읽기전용 pointer의 중간과정이 하나 있어야된다.
 */
fn ptr_to_jmp_buf() -> *mut i8 {
    unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jmp_buf();
    println!("jump {:p}", franken_pointer);
    unsafe { longjmp(franken_pointer) };
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
    register_signal_handler();

    let should_shut_down = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };

    unsafe {
        SHUT_DOWN = should_shut_down;
    }

    println!("raise");
    return_early();
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

fn print_depth(depth: usize) {
    for _ in 0..depth {
        print!("#");
    }
    println!();
}

fn dive(depth: usize, max_depth: usize) {
    unsafe {
        if SHUT_DOWN {
            println!("!");
            return;
        }
    }
    print_depth(depth);

    if depth >= max_depth {
        return;
    } else if depth == MOCK_SIGNAL_AT {
        unsafe {
            libc::raise(SIGUSR1);
        }
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

fn main() {
    const JUMP_SET: i32 = 0;

    println!("JMP_BUF_WIDTH: {}", JMP_BUF_WIDTH);
    register_signal_handler();

    let return_point = ptr_to_jmp_buf();
    println!("save {:p}", return_point);
    let rc = unsafe { setjmp(return_point) };
    println!("rc {:x}", rc);
    if rc == JUMP_SET {
        dive(0, 10);
    } else {
        println!("early return");
    }

    println!("finishing!");
}
