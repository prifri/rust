#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

static pthread_mutex_t barrier_mut = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t barrier_cond = PTHREAD_COND_INITIALIZER;

static void barrier(volatile int *cnt, int max)
{
	if (pthread_mutex_lock(&barrier_mut) != 0)
	{
		perror("pthread_mutex_lock");
		exit(-1);
	}

	(*cnt)++;

	if (*cnt == max)
	{
		if (pthread_cond_broadcast(&barrier_cond) != 0)
		{
			perror("pthread_cond_broadcast");
			exit(-1);
		}
	} else
	{
		do {
			if (pthread_cond_wait(&barrier_cond, &barrier_mut) != 0)
			{
				perror("pthread_cond_wiat");
				exit(-1);
			}
		} while (*cnt < max);
	}

	if (pthread_mutex_unlock(&barrier_mut) != 0)
	{
		perror("pthread_mutex_unlock");
		exit(-1);
	}
}

static void *worker(void *arg)
{
	static int num;
	barrier(&num, 10);
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
		pthread_join(tn[i], NULL);
	}
	return 0;
}

