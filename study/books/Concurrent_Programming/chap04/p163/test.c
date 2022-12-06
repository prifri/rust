#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/types.h>
#include <unistd.h>

static pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;

static void handler(int sig)
{
	printf("received signal: %d\n", sig);
	int cnt = 5;
	while (cnt--)
	{
		printf("signal handler hi\n");
		sleep(1);
	}
}

static void *sender(void *arg)
{
	pthread_detach(pthread_self());

	int dest_pid = (intptr_t)arg;
	sleep(1);
	printf("send\n");
	kill(dest_pid, SIGUSR1);
	int cnt = 10;
	while (cnt--)
	{
		printf("sender hi\n");
		sleep(1);
	}
	return NULL;
}

static void create_sig_sender(int dest_pid)
{
	pthread_t th;
	pthread_create(&th, NULL, sender, (void *)(intptr_t)dest_pid);
}

int main(void)
{
	pid_t pid = getpid();
	printf("pid : %d\n", pid);

	signal(SIGUSR1, handler);

	create_sig_sender(pid);

/*
 * prifri, 2022.12.06:
 * - signal handler를 받으면 main은 signal이 끝나기전까지 멈춘다.
 */
#if 0
	int cnt = 100;
	while (cnt--)
	{
		printf("main hi %d\n", cnt);
		volatile uint64_t d = 99999999;
		while (d--) {
		}
	}
#endif
/*
 * prifri, 2022.12.06:
 * - 일반 ubuntu 환경의 linux는 받아도 안깨어난다.
 */
	pthread_mutex_lock(&mutex);
	printf("wait\n");
	if (pthread_cond_wait(&cond, &mutex) != 0)
	{
		perror("pthread_cond_wait");
		exit(1);
	}
	printf("sprious wake up\n");
	pthread_mutex_unlock(&mutex);

	return 0;
}
