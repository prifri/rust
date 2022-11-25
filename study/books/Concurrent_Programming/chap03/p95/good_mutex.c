#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include <stdlib.h>

static bool test_and_set(bool *p)
{
	return __sync_lock_test_and_set(p, 1);
}

static void tas_release(volatile bool *p)
{
	return __sync_lock_release(p);
}

#define CNT 10
static uint64_t g_success[CNT];
static uint64_t g_fail[CNT];
static bool g_lock = false;
static void *f(void *a)
{
	int id = (intptr_t)a;
	uint32_t loop = 1000;
	while (loop--)
	{
retry:
		if (!test_and_set(&g_lock))
		{
			g_success[id]++;
			//usleep(rand() % 5);
			usleep(1);
		}
		else
		{
			g_fail[id]++;
			goto retry;
		}
		tas_release(&g_lock);
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
		printf("[%d] %ld %ld\n", i, g_success[i], g_fail[i]);
	}
}
