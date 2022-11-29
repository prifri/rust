#include <inttypes.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define HOLDTIME (1)
#define NUM_THREAD 8

#ifdef RWLOCK
static pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;
static void do_lock(void)
{
	pthread_rwlock_rdlock(&lock);
	for (uint64_t i = 0; i < HOLDTIME; i++)
	{
		asm volatile("nop");
	}
	pthread_rwlock_unlock(&lock);
}
#elif defined(RWLOCK_WR)
static pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;
static void do_lock(void)
{
	pthread_rwlock_wrlock(&lock);
	for (uint64_t i = 0; i < HOLDTIME; i++)
	{
		asm volatile("nop");
	}
	pthread_rwlock_unlock(&lock);
}
#elif defined(MUTEX)
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static void do_lock(void)
{
	pthread_mutex_lock(&lock);
	for (uint64_t i = 0; i < HOLDTIME; i++)
	{
		asm volatile("nop");
	}
	pthread_mutex_unlock(&lock);
}
#else
//empty
static void do_lock(void)
{
	for (uint64_t i = 0; i < HOLDTIME; i++)
	{
		asm volatile("nop");
	}
}
#endif

static void barrier(volatile int *cnt, int max)
{
	__sync_fetch_and_add(cnt, 1);
	while (*cnt < max);
}

static volatile int flag = 0;
static volatile int waiting_1 = 0;
static volatile int waiting_2 = 0;
static uint64_t count[NUM_THREAD - 1];

void *worker(void *arg)
{
	uint32_t id = (intptr_t)arg;
	barrier(&waiting_1, NUM_THREAD);

	uint64_t n = 0;
	while (flag == 0)
	{
		do_lock();
		n++;
	}
	count[id] = n;

	barrier(&waiting_2, NUM_THREAD);

	return NULL;
}

void *timer(void *arg)
{
	barrier(&waiting_1, NUM_THREAD);

	sleep(10);
	flag = 1;

	barrier(&waiting_2, NUM_THREAD);
	for (int i = 0; i < NUM_THREAD - 1; i++)
	{
		printf("%lu\n", count[i]);
	}

	return NULL;
}

int main(void)
{
#ifdef RWLOCK
	printf("rwlock\n");
#elif defined(RWLOCK_WR)
	printf("rwlock_wr\n");
#elif defined(MUTEX)
	printf("mutex\n");
#else
	printf("empty\n");
#endif

	for (uint32_t i = 0; i < NUM_THREAD - 1; i++)
	{
		pthread_t th;
		pthread_create(&th, NULL, worker, (void *)(intptr_t)i);
		pthread_detach(th);
	}

	pthread_t th;
	pthread_create(&th, NULL, timer, NULL);
	pthread_join(th, NULL);
}
