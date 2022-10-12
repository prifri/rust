
/*
 * prifri, 2022.10.12:
 * - 운영 체제 없이 동작하기위해 표준라이브러리와 main 함수 존재 등의 검사를
 *   끈다.
 * - no_std로 인해 Vec등은 이제 사용하지 못한다.
 */
#![no_std]
#![no_main]

/*
 * prifri, 2022.10.12:
 * - 불안정한 core_intrinsics API를 사용할 수 있게 해야한다.
 */
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]

/*
 * prifri, 2022.10.12:
 * - rust 심벌 명명 규칙을 비활성화한다.
 */
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    unsafe {
        framebuffer
            .offset(1)
            .write_volatile(0x30);
    }
    loop {}
}
