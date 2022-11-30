#include <stdbool.h>
#include <stdint.h>

/*
 * prifri, 2022.11.23:
 * - compile
 *   gcc -S -O2 -o asm.s test.c
 *   aarch64-linux-gnu-gcc -S -O2 -o asm.s test.c 
 *
 * - rdi, rsi, rdx
 *    0 ,   1,   2
 *
 * - x86-64
 
.LFB0:
	.cfi_startproc
	endbr64
	movzbl	(%rdi), %eax
	testb	%al, %al
	jne	.L1
	movb	$1, (%rdi)
.L1:
	ret

 * - arm64
 *
 *
test_and_set:
.LFB0:
	.cfi_startproc
	mov	x1, x0
	ldrb	w0, [x0]
	cbnz	w0, .L2
	mov	w2, 1
	strb	w2, [x1]
.L2:
	ret
 */
bool test_and_set(bool *p)
{
	if (*p)
	{
		return true;
	}
	else
	{
		*p = true;
		return false;
	}
}

/*
 * prifri, 2022.11.23:
 * - x86-64
 
test_and_set2:
.LFB1:
	.cfi_startproc
	endbr64
	movl	$1, %eax     ; eax = 1. return값을 미리 1로 설정.
	xchgb	(%rdi), %al  ; TAS. (%rdi), %al을 교환한다.
	testb	%al, %al	 ; 교환된 (%rdi)값이 0인지 검사해서
	setne	%al          ; 결과값을 %al에 다시 저장한ㄷ.
	ret                  ; %al은 결국 eax이므로 이값으로 return된다.

 * - al
 *   eax의 하위 8bit.
 * - xchgb (%rdi), %al
 *   %al은 movl $1, %eax에 의해 1로 초기화되있다. 여기서
 *   xchgb로 1byte만을 교환을 한다.
 *   xchg는 atomic 접근으로 두값을 교환한다.
 *
 * - arm64
 *
test_and_set2:
.LFB1:
	.cfi_startproc
	mov	w2, 1
.L5:
	ldxrb	w1, [x0]
	stxrb	w3, w2, [x0]
	cbnz	w3, .L5
	tst	w1, 255
	dmb	ish
	cset	w0, ne
	ret
 *
 * - x86-64처럼 전용 명령어를 쓴게아니라 LL/SC의 byte명령어로만 접근해서
 *   바꾸는 식으로 하는게 보인다.
 */
bool test_and_set2(bool *p)
{
	return __sync_lock_test_and_set(p, 1);
}

void tas_release(volatile bool *p)
{
	return __sync_lock_release(p);
}

int main(void)
{
	return 0;
}
