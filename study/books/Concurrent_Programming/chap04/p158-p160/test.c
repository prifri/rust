#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

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

static void spinlock_acquire(int id, volatile bool *lock)
{
	while (1)
	{
		while (*lock)
		{
		}

		if (!test_and_set(lock))
		{
			return;
		}
	}
}

static void spinlock_release(bool *lock)
{
	tas_release(lock);
}

struct reent_lock {
	bool lock;
	int id;
	int cnt;
};

static void reentlock_acquire(struct reent_lock *lock, int id)
{
	if (lock->lock && lock->id == id)
	{
		lock->cnt++;
		return;
	}

	spinlock_acquire(id, &lock->lock);
	lock->id = id;
	lock->cnt++;
}

static void reentlock_release(struct reent_lock *lock)
{
	lock->cnt--;
	if (lock->cnt != 0)
	{
		return;
	}

	lock->id = 0;
	spinlock_release(&lock->lock);
}

static struct reent_lock lock_var;
static void reent_lock_test(int id, int n)
{
	if (n == 0)
	{
		return;
	}

	reentlock_acquire(&lock_var, id);
	reent_lock_test(id, n - 1);
	reentlock_release(&lock_var);
}

static void *thread_func(void *arg)
{
	int id = (intptr_t)arg;
	assert(id != 0);
	for (int i = 0; i < 10000; i++)
	{
		reent_lock_test(id, 10);
	}
	assert(lock_var.cnt == 0);
	return NULL;
}

#define NUM_THREADS 5
int main(void)
{
	pthread_t v[NUM_THREADS];
	for (int i = 0; i < NUM_THREADS; i++)
	{
		pthread_create(&v[i], NULL, thread_func, (void *)(intptr_t)(i + 1));
	}

	for (int i = 0; i < NUM_THREADS; i++)
	{
		pthread_join(v[i], NULL);
	}

	return 0;
}
