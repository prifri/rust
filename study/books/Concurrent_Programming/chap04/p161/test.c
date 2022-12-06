#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <errno.h>
/*
 * prifri, 2022.12.06:
 * - .._NP를 쓸려면 __USE_GNU를 선언해줘야한다.
 * - NP의미
 *   https://stackoverflow.com/questions/15636850/what-is-the-difference-between-pthread-recursive-mutex-initializer-and-pthread-r 
 *   
 */
#define __USE_GNU
#include <pthread.h>

#define STATIC_INIT

static void test_fast_runtime(void)
{
	pthread_mutex_t fast_lock;

	pthread_mutex_init(&fast_lock, NULL);
	{
		pthread_mutex_lock(&fast_lock);
		{
#if 0 //dead lock
			pthread_mutex_lock(&fast_lock);
#endif
			printf("%s %d\n", __func__, __LINE__);
		}
		pthread_mutex_unlock(&fast_lock);
	}
	pthread_mutex_destroy(&fast_lock);
}

static void test_fast_init(void)
{
	pthread_mutex_t fast_lock = PTHREAD_MUTEX_INITIALIZER;

	pthread_mutex_lock(&fast_lock);
	{
#if 0 //dead lock
		pthread_mutex_lock(&fast_lock);
#endif
		printf("%s %d\n", __func__, __LINE__);
	}
	pthread_mutex_unlock(&fast_lock);
}

/*
 * prifri, 2022.12.06:
 * - https://wariua.github.io/man-pages-ko/pthread_mutex_lock%283%29/
 *   https://man7.org/linux/man-pages/man3/pthread_mutex_lock.3p.html
 */
static int check_lock(int ret, uint32_t line)
{
	switch (ret)
	{
#define CASE_RETURN(e, l)	\
		case e: printf(#e " : %d %d\n", ret, line); return ret
		CASE_RETURN(EAGAIN, line);
		CASE_RETURN(EINVAL, line);
		CASE_RETURN(ENOTRECOVERABLE, line);
		CASE_RETURN(EOWNERDEAD, line);
		CASE_RETURN(EDEADLK, line);
		CASE_RETURN(EBUSY, line);
		CASE_RETURN(EPERM, line);
		case 0:
			printf("success : %d\n", line);
			return 0;
		default:
			printf("unknown : %d %d\n", ret, line);
			return ret;
	}
}

static void test_recursive_runtime(void)
{
	pthread_mutexattr_t attr;
	pthread_mutex_t recursive_lock;

	pthread_mutexattr_init(&attr);
	pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_RECURSIVE);

	pthread_mutex_init(&recursive_lock, &attr);
	pthread_mutexattr_destroy(&attr);
	{
		pthread_mutex_lock(&recursive_lock);
		pthread_mutex_lock(&recursive_lock);
		{
			printf("%s %d\n", __func__, __LINE__);
		}
		int ret = check_lock(pthread_mutex_unlock(&recursive_lock), __LINE__);
		assert(ret == 0);
		ret = check_lock(pthread_mutex_unlock(&recursive_lock), __LINE__);
		assert(ret == 0);
	}
	pthread_mutex_destroy(&recursive_lock);
}

static void test_recursive_init(void)
{
	pthread_mutex_t recursive_lock = PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP;

	pthread_mutex_lock(&recursive_lock);
	pthread_mutex_lock(&recursive_lock);
	{
		printf("%s %d\n", __func__, __LINE__);
	}
	pthread_mutex_unlock(&recursive_lock);
	pthread_mutex_unlock(&recursive_lock);
}

static void test_errchk_runtime(void)
{
	pthread_mutexattr_t attr;
	pthread_mutex_t errchk_lock;

	pthread_mutexattr_init(&attr);
	pthread_mutexattr_settype(&attr, PTHREAD_MUTEX_ERRORCHECK);

	pthread_mutex_init(&errchk_lock, &attr);
	pthread_mutexattr_destroy(&attr);
	{
		int ret = check_lock(pthread_mutex_lock(&errchk_lock), __LINE__);
		assert(ret == 0);
		ret = check_lock(pthread_mutex_lock(&errchk_lock), __LINE__);
		if (ret == EDEADLK)
		{
			printf("%s %d deadlock detect\n", __func__, __LINE__);
			check_lock(pthread_mutex_unlock(&errchk_lock), __LINE__);
/*
 * prifri, 2022.12.06:
 * - 불필요. lock을 완전히 풀엇을때 무슨 에러를 return해주는지 확인
 */
			check_lock(pthread_mutex_unlock(&errchk_lock), __LINE__);
		} else
		{
			assert(0);
			check_lock(pthread_mutex_unlock(&errchk_lock), __LINE__);
			check_lock(pthread_mutex_unlock(&errchk_lock), __LINE__);
		}
	}
	pthread_mutex_destroy(&errchk_lock);
}

static void test_errchk_init(void)
{
	pthread_mutex_t errchk_lock = PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP;

	pthread_mutex_lock(&errchk_lock);
	pthread_mutex_lock(&errchk_lock);
	{
		printf("%s %d\n", __func__, __LINE__);
	}
	pthread_mutex_unlock(&errchk_lock);
	pthread_mutex_unlock(&errchk_lock);
}

int main(void)
{
	test_fast_runtime();
	test_fast_init();
	test_recursive_runtime();
	test_recursive_init();
	test_errchk_runtime();
	test_errchk_init();
	return 0;
}
