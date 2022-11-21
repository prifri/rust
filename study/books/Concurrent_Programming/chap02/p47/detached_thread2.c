#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define NUM_THREADS	10

void *thread_func(void *arg)
{
	pthread_detach(pthread_self());

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
	pthread_t th;
	if (pthread_create(&th, NULL, thread_func, NULL) != 0)
	{
		perror("pthread_create");
		return -1;
	}

	//thread 끝날때까지 wait
	sleep(7);

	return 0;
}
