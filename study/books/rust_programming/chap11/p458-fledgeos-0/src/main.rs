
/*
 * prifri, 2022.10.12:
 * - 운영 체제 없이 동작하기위해 표준라이브러리와 main 함수 존재 등의 검사를
 *   끈다.
 * - no_std로 인해 Vec등은 이제 사용하지 못한다.
 */
#![no_std]
/*
 * prifri, 2022.11.04:
 * - main()함수를 포함시키지 않는다.
 * - 일반 application이 실행될때, 즉 loader에 의해서 실행될때 최초의 함수로
 *   main을 찾고(정확히 여기선 compiler가 _start를 자동으로 만들고 _start에서
 *   main을 호출하는식 이라고 하는거 같다.)
 *   main, library가 아닌 excuable일때에는 최초의 함수가 필요하니
 *   당연히 있어야되지만, 현재 상황에서는 bootloader가 _start를 찾아서 실행을
 *   하는 상황이 main이 굳이 있을  필요가 없다.
 */
#![no_main]

/*
 * prifri, 2022.10.12:
 * - 불안정한 core_intrinsics API를 사용할 수 있게 해야한다.
 * - rust compiler의 백엔드는 llvm compiler를 사용하는데, llvm의 내부는
 * rust의 안정섣ㅇ 보장이 아니므로 rust에 제공되는 부분이 변결될 위험이 있다.
 * 그래서 nightly compiler를 사용하고 프로그램에서 명시적으로 불안정한 api를
 * 선택해야 함을 의미한다.
 */
#![feature(core_intrinsics)]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]

/*
 * prifri, 2022.10.12:
 * - rust 심벌 명명 규칙을 비활성화한다. rust에서는 name mangling이라는
 * process를 통해 심벌을 생성하여 이를 방지한다. 이를 disable한다.
 *
 * - 예를들어 memcpy라는 함수를 만들면 예를들어  memcpy_abc라고 rust에서 
 * 임의로 바꿔버린다. 이 경우 linker에서 memcpy만 찾기 때문에 못 찾는다.
 * 그래서 disable한다.
 */
#[no_mangle]
/*
 * prifri, 2022.11.04:
 * - rust는 반드시 panic을 처리해야 한다.
 * - rust에는 panic처리를 자체적으로 처리하는데(std에 포함), no_std를
 * 사용했으므로 이를 수동으로 구현을 해줘야한다.
 * - PanicInfo
 *   panic이 발생한 위치에 대한 정보가 있다.(file name, line number등)
 */
pub fn panic(_info: &PanicInfo) -> ! {
    /*
     * prifri, 2022.11.04:
     * - 종료.
     */
    intrinsics::abort();
}

#[no_mangle]
/*
 * prifri, 2022.11.04:
 * - extern "C"
 * c 호출 규칙을 선택한다.
 * - rust는 호츌 규칙을 지정하지 않는다고 한다(이 부분의 정확한 의미는 잘 모르겠다.)
 * - 부팅시, 즉 bootloader에서 _start를 찾아서 실행시킬시 해당 컴파일러의
 * 호출 규약을 지키며 (register를 함수로 넘기거나 stack의 사용여부에 대한 규칙)
 * 호출을 해줘야되는데, rust내에서는 따로 이런게 없나보다.
 *
 * - 호출 규약
 *   Calling Convention라고 큰 범위에서 불리는거 같은데 arm64에서는
 *   Procedure Call Standard라고 부른다
 *   (https://developer.arm.com/documentation/102374/0100/Procedure-Call-Standard)
 */
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
/*
 * prifri, 2022.11.04:
 * - 0xb8001로 설정. 0x30 은 배경을 청록색으로 하라는 의미.
 * - offset
 *   pointer유형(*mut u8등)에 크기만큼 주소 공간을 이동한다.
 *   *mut u32에 사용했다면 4byte만큼 이동할것이다.
 * - write_volatile
 *   memory에 강제로 기록.
 *   compiler에 따라 상수값들에 대한 memory write를 최적화할수 있다.
 *   ex1)
 *   for (int i = 0; i < 30; i++)
 *   {
 *     *0b8001 = 1;
 *   }
 *   최적화를 안하면 *0b8001을 30번쓰겟지만, 최적화를 한다면 *b8001을 한번만쓴다.
 *
 *   ex2)
 *   *b8001 = 3;
 *   ...
 *   *b8001 = 5;
 *   최적화를 안하면 3, 5가 순서대로 써지겠지만 최적화가 된다면 *b8001에 5가써진다
 *
 *   ex3)
 *
 *   *b8001 = 8;
 *   (이후로 *b8001에 write 안함 접근안함 or 순서가 변겅되 해당 시점이 아닌
 *   나중으로 밀림)
 *
 *   code상에 *b8001에 접근을 안하니 아에 code를 빼버린다. 하지만 실제 다른곳에서
 *   b8001을 접근할수도있다. 이런경우 volatile 선언을 해줘서 force write/read
 *   를 해줘야한다.
 */
    unsafe {
        framebuffer
            .offset(1)
            .write_volatile(0x30);
    }
    loop {}
}
