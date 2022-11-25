#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

static pthread_mutex_t mut = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static volatile bool ready = false;
char buf[256];

static void *producer(void *arg)
{
	printf("producer: ");

	while (fgets(buf, sizeof(buf), stdin) == NULL);

	pthread_mutex_lock(&mut);
	ready = true;

	if (pthread_cond_broadcast(&cond) != 0)
	{
		perror("pthread_cond_broadcast");
		exit(-1);
	}

	pthread_mutex_unlock(&mut);
	return NULL;
}

static void *consumer(void *arg)
{
	pthread_mutex_lock(&mut);

	while (!ready)
	{
		if (pthread_cond_wait(&cond, &mut) != 0)
		{
			perror("pthread_cond_wait");
			exit(-1);
		}
	}

	pthread_mutex_unlock(&mut);
	printf("consumer: %s\n", buf);
	return NULL;
}

int main(void)
{
	pthread_t pr, cn;
	pthread_create(&pr, NULL, producer, NULL);
	pthread_create(&cn, NULL, consumer, NULL);

	pthread_join(pr, NULL);
	pthread_join(cn, NULL);

	pthread_mutex_destroy(&mut);

	if (pthread_cond_destroy(&cond) != 0)
	{
		perror("pthread_cond_destroy");
		return -1;
	}

	return 0;
}
