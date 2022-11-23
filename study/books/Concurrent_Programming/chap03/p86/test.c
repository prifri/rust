#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

/*
 * prifri, 2022.11.23:
 * - x86-64
 
compare_and_swap:
.LFB23:
	.cfi_startproc
	endbr64
	xorl	%eax, %eax   ; xrol 명령으로 %eax 값을 0으로 설정. 
	cmpq	%rsi, (%rdi) ; %rsi == %rdi를 하고 결과가 flag에 저장.
	jne	.L1              ; != 였으면 goto .L1
	movq	%rdx, (%rdi) ; *p = newval
	movl	$1, %eax     : %eax = 1(true)로 설정
.L1:
	ret                  ; %eax 값을 들고 return.
	.cfi_endproc
  
 * - arm64
 *
compare_and_swap:
.LFB23:
	.cfi_startproc
	mov	x3, x0           ; p를 x3에 backup
	ldr	x0, [x0]         ; x0 = *p
	cmp	x0, x1           ; x0 == x1
	bne	.L3              ; != 이면 goto .L3
	mov	w0, 1            ; return값 true set.
	str	x2, [x3]         ; *p = x2
	ret                  ; return
.L3:
	mov	w0, 0
	ret                  ; return

 * - x86-64, arm64 둘다 atomic 명령어는 보이지 않는다.
 */

bool compare_and_swap(uint64_t *p, uint64_t val, uint64_t newval)
{
	if (*p != val)
	{
		return false;
	}
	*p = newval;
	return true;
}

/*
 * prifri, 2022.11.23:
 * - x86-64
 *
 *
compare_and_swap2:
.LFB24:
	.cfi_startproc
	endbr64
	movq	%rsi, %rax           ; %rax = %rsi
	lock cmpxchgq	%rdx, (%rdi) ; CAS
	sete	%al                  ; %cl = ZF flag
	ret
	.cfi_endproc

 * - cmpxchgq
 *   해당명령안에서 지정된 메모리에 해당하는 CPU 캐시 라인의 소유권이 배타적인것을
 *   보증한다.
 * - sete(Set Byte on Condition)
 *   ZF 클래스의 값을 cl에(sete 명령은 8bit register만 지정할수 있고,
 *   cl은 ecx의 하위 8bit에 해당함) 저장한다.
 *
 * - CPU 캐시라인 배타적 접근.
 *   아키텍처마다 다르겠지만 exclusive연산등을 사용한 hardware,
 *   cache protocol을 통해서 동기 접근을 한다..
 *
 * - arm64
 *
compare_and_swap2:
.LFB24:
	.cfi_startproc
.L6:
	ldxr	x3, [x0]
	cmp	x3, x1
	bne	.L7
	stlxr	w4, x2, [x0]
	cbnz	w4, .L6
.L7:
	cset	w0, eq
	dmb	ish
	ret
	.cfi_endproc
 * 
 * - ldxr / stlxr
 *   ldxr에서 read -> stlxr에서 write.
 *   이때 stlxr에서 w4가 0이 아니면( 다시 ldxr으로 가서 반복한다.
 *   이를 LL/SC방식이라고 한다.
 *   실제 arm8.1에서는 LSE를 지원하며, 이게 x86-64의 cmpxchgq와 비슷한 모양의
 *   CAS방식이다.(arm측에서는 x86-64와 다른방식의 구현이라고 주장한다고 한다.
 *   하지만 대충 cas라고 부른다.)
 *
 * - ldxr / stlxr 동작 원리
 *   ldxr을 할때 exclusive monitor(대충 하드웨어가 해준다는뜻)에 접근 표시를하고
 *   st(l)xr을 할때 접근 표시를 읽어 동기접근 여부를 확인한다. 실패햇으면
 *   성공할때까지 반복하게 된다.
 * - stlxr
 *   l은 release의 의미로 stlxr의 아래위 code를 나눠 code 순서를 지킨다는
 *   memory 장벽의 일종.
 *   거기다 다른 cpu cache 에도 변경사항을 notify 해준후 그 결과까지
 *   기다린다고 알고있다.
 * - x86-64의 CAS에 비해선 LL/SC가 느릴 순 있으나 다른 CPU로부터의 write 여부를
 *   검출 할 수 있다.
 */
bool compare_and_swap2(uint64_t *p, uint64_t val, uint64_t newval)
{
	return __sync_bool_compare_and_swap(p, val, newval);
}

int main(void)
{
	return 0;
}
