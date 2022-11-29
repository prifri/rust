#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include "../../utils/thread_info/thread_info.h"

/*
 * prifri, 2022.11.29:
 * - 돌려보면 pthread rw lock은 read 우선으로 깨우는걸 알수있다.
 */
#define T_READ_CNT	5
#if 0
#define T_WRITE_CNT	1
#else
#define T_WRITE_CNT	10
#endif

#define T_CNT (T_READ_CNT + T_WRITE_CNT)

static pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER;

static void do_write(uint32_t id)
{
	if (pthread_rwlock_wrlock(&rwlock) != 0)
	{
		perror("pthread_rwlock_rdlock");
		exit(-1);
	}

	thread_info_set_status(id, THREAD_STATUS_WRITE_ACQUIRE);
	sleep(1);
	thread_info_set_status(id, THREAD_STATUS_WRITE_RELEASE);

	if (pthread_rwlock_unlock(&rwlock) != 0)
	{
		perror("pthread_rwlock_unlock");
		exit(-1);
	}
}

static bool is_end;

static void *worker_write(void *arg)
{
	int id = (intptr_t)arg;
	int loop_cnt = 10;

	thread_info_set_status(id, THREAD_STATUS_START);
	thread_info_set_type(id, THREAD_TYPE_WRITER);
	while (loop_cnt-- && !is_end)
	{
		do_write(id);
		thread_info_set_status(id, THREAD_STATUS_WAIT);
		sleep(1);
	}
	is_end = true;
	thread_info_set_status(id, THREAD_STATUS_END);
	return NULL;
}

static void do_read(uint32_t id)
{
	if (pthread_rwlock_rdlock(&rwlock) != 0)
	{
		perror("pthread_rwlock_rdlock");
		exit(-1);
	}

	thread_info_set_status(id, THREAD_STATUS_READ_ACQUIRE);
	sleep(1);
	thread_info_set_status(id, THREAD_STATUS_READ_RELEASE);

	if (pthread_rwlock_unlock(&rwlock) != 0)
	{
		perror("pthread_rwlock_unlock");
		exit(-1);
	}
}

static void *worker_read(void *arg)
{
	int id = (intptr_t)arg;
	thread_info_set_type(id, THREAD_TYPE_READER);
	thread_info_set_status(id, THREAD_STATUS_START);
	sleep(2);
	int loop_cnt = 10;

	while (loop_cnt-- && !is_end)
	{
		do_read(id);
		thread_info_set_status(id, THREAD_STATUS_WAIT);
		if (is_end)
		{
			break;
		}
		sleep(1);
	}
	is_end = true;
	thread_info_set_status(id, THREAD_STATUS_END);
	return NULL;
}

typedef enum {
	THREAD_IDX_READ_START,
	THREAD_IDX_WRITE_START = T_READ_CNT,
	THREAD_IDX_MAX = T_CNT
} THREAD_IDX;

int main(void)
{
	thread_info_init(T_CNT);

	pthread_t tn[T_CNT];

	for (int i = THREAD_IDX_READ_START; i < THREAD_IDX_WRITE_START; i++)
	{
		if (pthread_create(&tn[i], NULL, worker_read, (void *)(intptr_t)i) != 0)
		{
			perror("pthread_create");
			return -1;
		}
	}

	for (int i = THREAD_IDX_WRITE_START; i < THREAD_IDX_MAX; i++)
	{
		if (pthread_create(&tn[i], NULL, worker_write, (void *)(intptr_t)i) != 0)
		{
			perror("pthread_create");
			return -1;
		}
	}

	for (int i = 0; i < T_CNT; i++)
	{
		pthread_join(tn[i], NULL);
	}

	thread_info_deinit();
	return 0;
}

