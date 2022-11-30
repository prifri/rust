#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

static bool test_and_set(volatile bool *p)
{
	return __sync_lock_test_and_set(p, 1);
}

static void tas_release(volatile bool *p)
{
	return __sync_lock_release(p);
}

#define CNT 10
static uint64_t g_success[CNT];
static uint64_t g_spin_cnt[CNT];
static bool g_lock = false;

static void spinlock_aqcuire(volatile bool *lock, uint32_t id)
{
	while (1)
	{
		while (*lock)
		{
			g_spin_cnt[id]++;
			//arm의 경우 wfi, wfe를 쓰면서 쉬어준다.
			//너무 spin을 하다보면 발열등에도 안좋다.
		}

		if (!test_and_set(lock))
			return;
	}
}

static void spinlock_release(bool *lock)
{
	tas_release(lock);
}

static void *f(void *a)
{
	int id = (intptr_t)a;
	uint32_t loop = 1000;
	while (loop--)
	{
		spinlock_aqcuire(&g_lock, id);
		//**************************************************
		//*** critical section 
		//usleep(rand() % 5);

		/*
		 * 크리티컬 섹션안에서 처리량이 많을 경우
		 * 1. wait하는 cpu가 spin하는 양이 많아진다.
		 * 2. 크리티컬 섹션내에서 contex switch가 이루어져 다른 프로세스에
		 * cpu 리소스를 주게되어 대기 상태가 되버리는 경우가 있다. 이 경우
		 * 대기시간이 압도적으로 길어져 패널티가 클것이다.
		 */
		g_success[id]++;
		usleep(1);

		//*** critical section 
		//**************************************************
		spinlock_release(&g_lock);
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
		printf("[%d] %ld %ld\n", i, g_success[i], g_spin_cnt[i]);
	}
}
