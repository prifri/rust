#include <pthread.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <semaphore.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREAD	10
#define NUM_LOOP	10
#define SEMA_NAME	"/mysemaphore"
#define MAX_SEMA_CNT	3

static int count = 0;

static void *th(void *th)
{
	sem_t *s = sem_open(SEMA_NAME, 0);
	if (s == SEM_FAILED)
	{
		perror("sem_open");
		exit(1);
	}

	for (int i = 0; i < NUM_LOOP; i++)
	{
		if (sem_wait(s) == -1)
		{
			perror("sem_wait");
			exit(1);
		}

		int old = __sync_fetch_and_add(&count, 1);
		printf("count = %d\n", old + 1);

		usleep(10 * 1000);

		__sync_fetch_and_sub(&count, 1);

		if (sem_post(s) == -1)
		{
			perror("sem_post");
			exit(1);
		}
	}

	if (sem_close(s) == -1)
	{
		perror("sem_close");
	}

	return NULL;
}

int main(void)
{
	sem_t *s = sem_open(SEMA_NAME, O_CREAT, 0660, MAX_SEMA_CNT);
	if (s == SEM_FAILED)
	{
		perror("sem_open");
		return 1;
	}

	pthread_t v[NUM_THREAD];
	for (int i = 0; i < NUM_THREAD; i++)
	{
		pthread_create(&v[i], NULL, th, NULL);
	}

	for (int i = 0; i < NUM_THREAD; i++)
	{
		pthread_join(v[i], NULL);
	}

	if (sem_close(s) == -1)
	{
		perror("sem_close");
	}

	if (sem_unlink(SEMA_NAME) == -1)
	{
		perror("sem_unlink");
	}

	return 0;
}
