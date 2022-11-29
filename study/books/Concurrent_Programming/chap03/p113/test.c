#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include "../../utils/thread_info/thread_info.h"

#define T_READ_CNT	5
#if 1
#define T_WRITE_CNT	1
#else
#define T_WRITE_CNT	10
#endif

#define T_CNT (T_READ_CNT + T_WRITE_CNT)

//***************************************************************************
// p98
//***************************************************************************

static bool test_and_set(volatile bool *p)
{
	return __sync_lock_test_and_set(p, 1);
}

static void tas_release(volatile bool *p)
{
	return __sync_lock_release(p);
}

static void spinlock_acquire(uint32_t id, volatile bool *lock)
{
	while (1)
	{
		while (*lock)
		{
			thread_info_set_status(id, THREAD_STATUS_SPIN);
		}

		if (!test_and_set(lock))
		{
			thread_info_set_status(id, THREAD_STATUS_SPIN_ACQUIRE);
			return;
		}
	}
}

static void spinlock_release(bool *lock)
{
	tas_release(lock);
}

//***************************************************************************
// p113
//***************************************************************************

/*
 * prifri, 2022.11.29:
 * - write가 많이 일어날 경우 영원히 read를 못 얻는 경우가 생길수있다.
 */
static void rwlock_read_acquire(uint32_t id, int *rcnt, volatile int *wcnt)
{
	while (1)
	{
		while (*wcnt)
		{
			thread_info_set_status(id, THREAD_STATUS_READ_SPIN);
		}

		__sync_fetch_and_add(rcnt, 1);
		if (*wcnt == 0)
		{
			thread_info_set_status(id, THREAD_STATUS_READ_ACQUIRE);
			break;
		}
		__sync_fetch_and_sub(rcnt, 1);
	}
}

static void rwlock_read_release(uint32_t id, int *rcnt)
{
	__sync_fetch_and_sub(rcnt, 1);
	thread_info_set_status(id, THREAD_STATUS_READ_RELEASE);
}

static void rwlock_write_acquire(uint32_t id,
								 bool *lock, volatile int *rcnt, int *wcnt)
{
	__sync_fetch_and_add(wcnt, 1);
	while (*rcnt)
	{
		thread_info_set_status(id, THREAD_STATUS_WRITE_SPIN);
	}
	spinlock_acquire(id, lock);
	thread_info_set_status(id, THREAD_STATUS_WRITE_SPIN_ACQUIRE);
}

static void rwlock_write_release(uint32_t id, bool *lock, int *wcnt)
{
	spinlock_release(lock);
	__sync_fetch_and_sub(wcnt, 1);
	thread_info_set_status(id, THREAD_STATUS_WRITE_SPIN_RELEASE);
}

static int rcnt, wcnt;
static bool lock = false;
static bool is_end = false;

static void do_write(uint32_t id, const char *f, bool *lock, int *rcnt, int *wcnt)
{
	rwlock_write_acquire(id, lock, rcnt, wcnt);
	{
		sleep(1);
	}
	rwlock_write_release(id, lock, wcnt);
}

static void *worker_write(void *arg)
{
	int id = (intptr_t)arg;
	int loop_cnt = 10;

	thread_info_set_status(id, THREAD_STATUS_START);
	thread_info_set_type(id, THREAD_TYPE_WRITER);
	while (loop_cnt-- && !is_end)
	{
		do_write(id, __func__, &lock, &rcnt, &wcnt);
		thread_info_set_status(id, THREAD_STATUS_WAIT);
		sleep(1);
	}
	is_end = true;
	thread_info_set_status(id, THREAD_STATUS_END);
	return NULL;
}

static void do_read(uint32_t id, const char *f, int *rcnt, int *wcnt)
{
	rwlock_read_acquire(id, rcnt, wcnt);
	{
		sleep(1);
	}
	rwlock_read_release(id, rcnt);
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
		do_read(id, __func__, &rcnt, &wcnt);
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

