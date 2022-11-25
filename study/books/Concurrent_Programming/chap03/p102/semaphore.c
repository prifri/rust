#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

#define __stringify_1(x...)	#x
#define __stringify(x...)	__stringify_1(x)
#define asm_volatile_goto(x...)	do { asm goto(x); asm (""); } while (0)

#define SEMA_NUM	4
#define CNT 10
static uint64_t g_success[CNT][SEMA_NUM];
static uint64_t g_spin_cnt[CNT];
static int g_cnt;

static uint32_t semaphore_aqcuire(volatile int *cnt, uint32_t id)
{
	uint32_t cnt_value;
	int tmp;
	/*
	 * prifri, 2022.11.25:
	 * - asm에 volatile 선언이 반드시 필요하다.
	 *   asm문만 봤을때 같은 주소를 계속 읽는것처럼 보여서
	 *   compiler가 최적화버리고 무한루프에 빠진다.
	 * - 책에선 b.lo이런거 썻는데 헷갈려서 lt 로 대체.
	 * - count 이전값으로 통계를 작성하니 그걸 위해 mov로 이전값을 backup
	 *   해서 쓴다.
	 * - w0가 ret의 retun 값.
	 * - +
	 *   read / write 둘다 가능
	 * - write only
	 *   read only
	 *   여기서 write only라는건 read가 불가능하다는게 아니라, write로
	 *   시작한다는 의미로 보는게 정확할듯. 즉 값을 초기화 안하고 사용해도된다.
	 *   는것
	 * - ret을 안쓰는 이유
	 *   c에서 return안쓰고 ret을써서 하는법을 모르겠다. compile warning..
	 *
	 * - 사실 ldxr / stlxr을 써도된다고 생각한다.
	 * -------------
	 *  - https://developer.arm.com/documentation/102336/0100/Load-Acquire-and-Store-Release-instructions
	 *  - ldxr / stxr vs ldar / stlr vs ldaxr / stlxr
	 *
	 *  - ldar / stlr
	 *  ldar / stlr은 atomic과는 관계없지만 ldxr / stxr과 스펠링이 비슷해
	 *  헷갈릴 수 있으므로 같이 정리한다.
	 *
	 *   1) ldar
	 *
	 *   다음과 같이 code가 이루어졌다고 가정한다.
	 *
	 *        Access A.
	 *   ======= ldar ========
	 *        Access B.
	 *
	 *   Access) ldr / str명령어류. memory access를 의미한다.
	 *
	 *   이 경우 Access A는 ldar을 넘을수있지면, ldar이후의 Access는
	 *   ldar 이후 적용되도록 보장한다.
	 *
	 *         Access A  ---+
	 *                      |
	 *   ======= ldar ======|====
	 *            ^         |
	 *            |         |
	 *         Access B     |
	 *                      v
	 *
	 *  2) stlr
	 *
	 *   다음과 같이 code가 이루어졌다고 가정한다.
	 *
	 *        Access A.
	 *   ======= stlr ========
	 *        Access B.
	 *
	 *   이 경우 Access B는 stlr을 넘을수있지면, stlr이전의 Access는것
	 *   stlr실행까지 적용되도록 보장한다.
	 *
	 *        Access A.  ---+
	 *                      |
	 *   ======= stlr ======|==
	 *           ^          |
	 *           |          |
	 *        Access B      |
	 *                      v
	 *
	 *  3) ldar / stlr
	 *
	 *        Access A.
	 *   ======= ldar ========
	 *        Access B.
	 *   ======= stlr ========
	 *        Access C.
	 *
	 *  위 ldar, stlr을 적용하면 결국 다음과 같아진다.
	 *
	 *         Access A  ---+
	 *                      |
	 *  ======== ldar ======|====
	 *   ^                  |
	 *   |     Access B     |
	 *   |                  v
	 *  =|====== stlr =========
	 *   |                   
	 *   +--- Access C       
	 *
	 * 4) ldar / stlr 의 그룹끼리
	 *
	 *  ^  Access A 
	 *  |     |
	 *  |     v
	 * =|==== stlr ==========
	 *  |             ^
	 *  +- Access B   | 
	 *                |
	 * ============= ldar ===
	 *        ^
	 *        |
	 *     Access C
	 *
	 * ldar은 stlr을 넘어서 실행이 안된다.
	 * 이것이 sequentially consistent.
	 *
	 * 5) ldapr
	 *
	 *  ^  Access A 
	 *  |     |
	 *  |     v       ^
	 * =|==== stlr X =|========
	 *  |             |
	 *  +- Access B   | 
	 *                |
	 * ============= ldapr Y ===
	 *        ^
	 *        |
	 *     Access C
	 *
	 * stlr과 ldapr이 다른 address 접근할경우 뛰어 넘을수있다는것.
	 * Rcpc의 개념으로 설명하는데 smp환경에서 value가 변경될경우 특정 시간이후로
	 * cpu들이 value을 관찰했을때 모두 일치하는 값을 가지는데에 대한 내용으로,
	 * weakness 의존성과도 연결지어서 말한다.
	 * 기존 ldar / stlr의 세트끼리는 sequentialy 적, 즉 강한 의존성을 가지는데,
	 * 이보다 더 낮으 의존성을 지닌 개념이된다.
	 *
	 * arm site에는 value가 달라야 ldapr이 stlr을 넘어선다는 얘기는 없지만
	 * ldapr에 대한 좀더 자세한 문서를 보면 해당내용이 있었던 거같다.
	 */
	asm volatile (
		"1: ldr %w0, [%1]\n"
		"	cmp %w0, #"__stringify(SEMA_NUM)"\n"
		"	b.hi 1b\n"
		"2: ldaxr %w0, [%1]\n"
		"	cmp %w0, #"__stringify(SEMA_NUM)"\n"
		"	b.lt 3f\n"
		"	clrex\n"
		"	b 2b\n"
		"3: add %w0, %w0, #1\n"
		"	stlxr %w2, %w0, [%1]\n"
		"	cbnz %w2, 2b\n"
		"   sub %w0, %w0, #1\n"
		: "=r"(cnt_value), "+r"(cnt), "=r"(tmp));
	return cnt_value;
}

static void semaphore_release(int *cnt)
{
	__sync_fetch_and_sub(cnt, 1);
}

static void *f(void *a)
{
	int id = (intptr_t)a;
	uint32_t loop = 1000;
	while (loop--)
	{
		uint32_t curr_cnt = semaphore_aqcuire(&g_cnt, id);
		//**************************************************
		//*** critical section 
		//usleep(rand() % 5);

		if (curr_cnt >= SEMA_NUM)
		{
			printf("error %d\n", curr_cnt);
			curr_cnt = SEMA_NUM - 1;
		}
		g_success[id][curr_cnt]++;
		usleep(1);

		//*** critical section 
		//**************************************************
		semaphore_release(&g_cnt);
	}

	return NULL;
}

int main (void)
{
	srand(0);
	pthread_t tid[CNT];
	for (int i = 0; i < CNT; i++)
	{
		pthread_create(&tid[i], NULL, f, (void *)(intptr_t)i);
	}

	for (int i = 0; i < CNT; i++)
	{
		pthread_join(tid[i], NULL);
	}

	for (int i = 0; i < CNT; i++)
	{
		uint64_t total = 0;
		for (int j = 0; j < SEMA_NUM; j++)
		{
			printf("\t[%d] %ld\n", j, g_success[i][j]);
			total += g_success[i][j];
		}
		printf("[%d] %ld %ld\n", i, g_spin_cnt[i], total);
	}
}
