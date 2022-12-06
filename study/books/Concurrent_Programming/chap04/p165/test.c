#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/types.h>
#include <unistd.h>

static pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
static sigset_t set;

static void *handler(void *arg)
{
	pthread_detach(pthread_self());

	int sig;
	while (1)
	{
		if (sigwait(&set, &sig) != 0)
		{
			perror("sigwait");
			exit(1);
		}
		printf("%s %d lock ready\n", __func__, __LINE__);
		pthread_mutex_lock(&mutex);
		{
			printf("%s %d lock\n", __func__, __LINE__);
			sleep(1);
		}
		pthread_mutex_unlock(&mutex);
		printf("%s %d unlock\n", __func__, __LINE__);
	}
}

static void *worker(void *arg)
{
	int dest_pid = (intptr_t)arg;

	for (int i = 0; i < 10; i++)
	{
		printf("%s %d lock ready\n", __func__, __LINE__);
		pthread_mutex_lock(&mutex);
		printf("%s %d lock\n", __func__, __LINE__);
		kill(dest_pid, SIGUSR1);
		usleep(1000 * 500);
		pthread_mutex_unlock(&mutex);
		printf("%s %d unlock\n", __func__, __LINE__);
		sleep(1);
	}

	return NULL;
}

int main(void)
{
	pid_t pid = getpid();
	printf("pid : %d\n", pid);

	sigemptyset(&set);
	sigaddset(&set, SIGUSR1);
	if (pthread_sigmask(SIG_BLOCK, &set, NULL) != 0)
	{
		perror("pthread_sigmask");
	}

	pthread_t th, wth;
	pthread_create(&th, NULL, handler, NULL);
	pthread_create(&wth, NULL, worker, (void *)(intptr_t)pid);

	//create_sig_sender(pid);

/*
 * prifri, 2022.12.06:
 * - signal을 사용하여 signal handler를 등록하며 signal을 받았으면
 *   main은 signal이 끝나기전까지 멈췃었다.
 * - 그게 아니라 signal를 thread가 수행하게 하면 main은 안멈추고 동작한다.
 *
 * - 책에서는 signal()를 사용하면 main이 멈추고, 이때 main이 lock을 얻은
 *   상태였다면, signal handler에서 lock을 얻을시 deadlock에 빠진다는 설명이
 *   있엇어야 됫을거 같다.
 *
 * - 책에서는 lock을 얻는것도 기존 main에서 thread로 옮긴다.
 *   그냥 책을 보는 관점에서 헷갈릴수밖에 없는 상황이다.
 *   thread에서 lock을 얻고, signal()을 사용해서 signal handler에서도
 *   lock을 얻는다면 deadlock은 발생하지 않는다. signal handler가 동작중이여도
 *   main만 멈추고 thread는 동작하기 때문이다.
 *   
 *   즉 중요한점 어떤상황에서 main이 멈추냐, 안멈추냐이며 정리하면 다음과 같다.
 *
 *   1. signal()을 사용하여 signal handler를 이용할때, signal을 사용하면
 *   main이 멈춘다. 이때 main이 lock을 가지고 있는 상태에서 signal handler에서도
 *   lock을 얻으면 deadlock이 발생한다.
 *   2. main이 아니라 thread에서 lock을 얻으면 signal handler에서도 lock을써도
 *   deadlock에 안걸린다.
 *   3. signal()을 사용하여 signal handler를 등록하는 법 이외에, sigset_t
 *   구조체를 사용하여 특정 thread에 sig제어 권한을 넘겨주는 방법이 존재한다.
 *   이 방법을 사용하면 main이 안멈춘다.
 */
	int cnt = 10;
	while (cnt--)
	{
		printf("main lock ready %d\n", cnt);
		pthread_mutex_lock(&mutex);
		printf("main lock %d\n", cnt);
		sleep(1);
		pthread_mutex_unlock(&mutex);
		printf("main unlock %d\n", cnt);
		sleep(1);
	}

	pthread_join(wth, NULL);
	return 0;
}
