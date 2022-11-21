#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS	10

void *thread_func(void *arg)
{
	int id = (intptr_t)arg;
	for (int i = 0; i < 5; i++)
	{
		printf("id = %d, i = %d\n", id, i);
		sleep(1);
	}
	return NULL;
}

int main(void)
{
	pthread_attr_t attr;
	if (pthread_attr_init(&attr) != 0)
	{
		perror("pthread_attr_init");
		return -1;
	}

	if (pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED) != 0)
	{
		perror("pthread_attr_setdetachstate");
		return -1;
	}

	pthread_t th;
	if (pthread_create(&th, &attr, thread_func, NULL) != 0)
	{
		perror("pthread_create");
		return -1;
	}

	if (pthread_attr_destroy(&attr) != 0)
	{
		perror("pthread_attr_destroy");
		return -1;
	}

	//thread 끝날때까지 wait
	sleep(7);

	return 0;
}
