#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

#define SEMA_NUM	4
#define CNT 10
static uint64_t g_success[CNT][SEMA_NUM];
static uint64_t g_spin_cnt[CNT];
static int g_cnt;

/*
 * prifri, 2022.11.24:
 * - 세마포어는 process lock으로 보통 쓰지 않는다.
 *   process가 lock을 걸고 죽었다는걸 알 방법이 없기 때문이다.
 */
static int semaphore_aqcuire(volatile int *cnt, uint32_t id)
{
	while (1)
	{
		while (*cnt >= SEMA_NUM)
		{
			g_spin_cnt[id]++;
		}

		int curr = __sync_fetch_and_add(cnt, 1);
/*
 * prifri, 2022.11.24:
 * - *cnt로 해서 다시 가져오는건 바람직하지 않다.
 *   __sync_fetch_and_add 자체에 atomic적으로 이전값을 return해준다.
 */
		if (curr < SEMA_NUM)
		{
			return curr;
		}

		__sync_fetch_and_sub(cnt, 1);
	}
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
		int curr_cnt = semaphore_aqcuire(&g_cnt, id);
		//**************************************************
		//*** critical section 
		//usleep(rand() % 5);

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
