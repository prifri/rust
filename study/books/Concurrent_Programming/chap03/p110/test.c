#include <stdio.h>
#include <pthread.h>

static volatile int num = 0;

/*
 * prifri, 2022.11.25:
 * - max까지 spin돈다.
 * - barrier의 개념이 왜 wait가 되는지?
 *   barrier란게 결국 특정 시각에 모든 cpu가 특정 memory address 를 똑같은
 *   value로 봐야 한다는 개념이다.
 *   address에 value가 write를 한다는 개념은 write를 한순에 memory에
 *   해당 값이 써지는게 아니다. 만약 DRAM까지 써여된다면
 *
 *   cpu -> L1 -> L2 -> L3 -> DRAM
 *
 *   이런식으로 갈것이고, 보통은 L1에서 멈출것이다.
 *
 *   0x10 번지에 0이였던것을 1로 write라고 했다면
 *
 *   cpu -(wrtie)-> L1 (0x10 : 1) ....  DRAM (0x10 : 0)
 *
 *   이때 cpu2가 0x10을 read를 한다 했으면
 *
 *   DRAM (0x10 :0) - .. -> cpu2 (0x10 : 0)
 *   
 *   즉 cpu와 cpu2의 0x10이 보는게 다르다.
 *
 *   즉 0x10이 변경됬다는걸 다른 cpu에도 알려서 해당 값이 다 wrtie되고 read가
 *   완료 될때까지 wait해야된다.
 *
 *   이런 개념에서 결국 barrier란게 특정 flag를 관찰하는 wait가 된다.
 */
static void barrier(volatile int *cnt, int max)
{
	__sync_fetch_and_add(cnt, 1);
	while (*cnt < max);
}

static void *worker(void *arg)
{
	barrier(&num, 10);
	return NULL;
}

int main(void)
{
#define T_CNT 10
	pthread_t tn[T_CNT];
	for (int i = 0; i < T_CNT; i++)
	{
		if (pthread_create(&tn[i], NULL, worker, NULL) != 0)
		{
			perror("pthread_create");
			return -1;
		}
	}

	for (int i = 0; i < T_CNT; i++)
	{
		pthread_join(tn[i], NULL);
	}
	return 0;
}

