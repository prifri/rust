#include <stdio.h>
#include <pthread.h>

static volatile int num = 0;

/*
 * prifri, 2022.11.25:
 * - max까지 spin돈다.
 */
static void barrier(volatile int *cnt, int max)
{
	__sync__fetch_and_add(cnt, 1);
	while (*cnt < max);
}

static void *worker(void *arg)
{
	barrier(&num. 10);
	return NULL;
}

int main(void)
{
#define T_CNT 10
	pthread_t tn[T_CNT];
	for (int i = 0; i < T_CNT; i++)
	{
		if (pthread_create(&tn[i], NULL, worker, NULL) != 0)
		{
			perror("pthread_create");
			return -1;
		}
	}

	for (int i = 0; i < T_CNT; i++)
	{
		pthread_join(tn[i]);
	}
	return 0;
}

