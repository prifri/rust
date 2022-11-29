#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <unistd.h>
#include "thread_info.h"

#define T_READ_CNT	5
#if 1
#define T_WRITE_CNT	1
#else
#define T_WRITE_CNT	10
#endif

#define T_CNT (T_READ_CNT + T_WRITE_CNT)

//***************************************************************************
// stat
//***************************************************************************

#define ANSI_INIT               "\033[0m"
#define ANSI_BLINK              "\033[5m"
#define ANSI_BLACK              "\033[0;30m"
#define ANSI_RED                "\033[0;31m"
#define ANSI_GREEN              "\033[0;32m"
#define ANSI_YELLOW             "\033[0;33m"  
#define ANSI_BLUE               "\033[0;34m"
#define ANSI_MAGENTA            "\033[0;35m"  
#define ANSI_CYAN               "\033[0;36m"  
#define ANSI_DEFAULT            "\033[0;39m"

#define ANSI_LGRAY              "\033[0;37m"
#define ANSI_GRAY              "\033[0;90m"  
#define ANSI_BRED               "\033[0;91m"  
#define ANSI_BGREEN             "\033[0;92m"  
#define ANSI_BYELLOW            "\033[0;93m"  
#define ANSI_BBLUE              "\033[0;94m"  
#define ANSI_BMAGENTA           "\033[0;95m"  
#define ANSI_BCYAN              "\033[0;96m" 
#define ANSI_BWHITE              "\033[0;97m"

typedef struct {
	THREAD_TYPE type;
	THREAD_STATUS curr;
	uint64_t stat[THREAD_STATUS_MAX];
} thread_info;

static thread_info g_tinfo[T_CNT];

static const char *get_thread_status_str(THREAD_STATUS status)
{
	switch (status)
	{
		case THREAD_STATUS_START:
			return "GO";
		case THREAD_STATUS_WAIT:
			return "WAIT";
		case THREAD_STATUS_SPIN:
			return "S";
		case THREAD_STATUS_WRITE_SPIN:
			return "WS";
		case THREAD_STATUS_READ_SPIN:
			return "RS";
		case THREAD_STATUS_SPIN_ACQUIRE:
			return "Sa";
		case THREAD_STATUS_READ_ACQUIRE:
			return "Ra";
		case THREAD_STATUS_WRITE_ACQUIRE:
			return "Wa";
		case THREAD_STATUS_WRITE_SPIN_ACQUIRE:
			return "WSa";
		case THREAD_STATUS_SPIN_RELEASE:
			return "SL";
		case THREAD_STATUS_READ_RELEASE:
			return "RL";
		case THREAD_STATUS_WRITE_RELEASE:
			return "WL";
		case THREAD_STATUS_WRITE_SPIN_RELEASE:
			return "WSL";
		case THREAD_STATUS_END:
			return "END";
		case THREAD_STATUS_MAX:
		default:
			assert(0);
	}
}

static const char *get_thread_status_color(THREAD_STATUS status)
{
	switch (status)
	{
		case THREAD_STATUS_START:
		case THREAD_STATUS_WAIT:
			return ANSI_DEFAULT;
		case THREAD_STATUS_SPIN:
		case THREAD_STATUS_WRITE_SPIN:
		case THREAD_STATUS_READ_SPIN:
			return ANSI_RED;
		case THREAD_STATUS_SPIN_ACQUIRE:
		case THREAD_STATUS_READ_ACQUIRE:
		case THREAD_STATUS_WRITE_ACQUIRE:
		case THREAD_STATUS_WRITE_SPIN_ACQUIRE:
			return ANSI_GREEN;
		case THREAD_STATUS_SPIN_RELEASE:
		case THREAD_STATUS_READ_RELEASE:
		case THREAD_STATUS_WRITE_RELEASE:
		case THREAD_STATUS_WRITE_SPIN_RELEASE:
			return ANSI_BLUE;
		case THREAD_STATUS_END:
			return ANSI_MAGENTA;
		default:
			assert(0);
	}
}

static const char *get_thread_type_str(THREAD_TYPE type)
{
	switch (type)
	{
		case THREAD_TYPE_NONE:
			return "N";
		case THREAD_TYPE_READER:
			return "R";
		case THREAD_TYPE_WRITER:
			return "W";
		case THREAD_TYPE_BOTH:
			return "B";
		default:
			assert(0);
	}
}

static const char *get_thread_type_color(THREAD_TYPE type)
{
	switch (type)
	{
		case THREAD_TYPE_NONE:
			return ANSI_DEFAULT;
		case THREAD_TYPE_READER:
			return ANSI_RED;
		case THREAD_TYPE_WRITER:
			return ANSI_BLUE;
		case THREAD_TYPE_BOTH:
			return ANSI_YELLOW;
		default:
			assert(0);
	}
}

static void __show_table_name(void)
{
	for (THREAD_STATUS i = 0; i < THREAD_STATUS_MAX; i++)
	{
		printf(" %4s |", get_thread_status_str(i));
	}
	printf("\n");
}

static void __show_table_stat(uint32_t id)
{
	const thread_info *tinfo = &g_tinfo[id];
	for (THREAD_STATUS i = 0; i < THREAD_STATUS_MAX; i++)
	{
		uint64_t cnt = tinfo->stat[i];
		if (cnt < 1000)
		{
			printf(" %4lu |", cnt);
			continue;
		}

		cnt /= 1000;
		if (cnt < 1000)
		{
			printf(" %3luk |", cnt);
			continue;
		}

		cnt /= 1000;
		if (cnt < 1000)
		{
			printf(" %3luM |", cnt);
			continue;
		}

		cnt /= 1000;
		if (cnt < 1000)
		{
			printf(" %3luG |", cnt);
			continue;
		}

		cnt /= 1000;
		if (cnt < 1000)
		{
			printf(" %3luT |", cnt);
			continue;
		}

		cnt /= 1000;
		printf(" %3luP |", cnt);
	}
	printf("\n");
}

const char *thread_info_get_type_str(uint32_t id)
{
	const thread_info *tinfo = &g_tinfo[id];
	return get_thread_type_str(tinfo->type);
}

const char *thread_info_get_type_color(uint32_t id)
{
	const thread_info *tinfo = &g_tinfo[id];
	return get_thread_type_color(tinfo->type);
}

const char *thread_info_get_status_str(uint32_t id)
{
	const thread_info *tinfo = &g_tinfo[id];
	return get_thread_status_str(tinfo->curr);
}

const char *thread_info_get_status_color(uint32_t id)
{
	const thread_info *tinfo = &g_tinfo[id];
	return get_thread_status_color(tinfo->curr);
}


void thread_info_show_table(void)
{
	printf("| %3s | %5s | %5s |", "idx", "type", "curr");
	__show_table_name();

	for (int i = 0; i < T_CNT; i++)
	{
		printf("| %3d | %s%5s%s | %s%5s%s |",
			   i,
			   thread_info_get_type_color(i),
			   thread_info_get_type_str(i),
			   ANSI_DEFAULT,
			   thread_info_get_status_color(i),
			   thread_info_get_status_str(i),
			   ANSI_DEFAULT);
		__show_table_stat(i);
	}

	printf("\n");
}

void thread_info_set_status(const uint32_t id, THREAD_STATUS status)
{
	thread_info *tinfo = &g_tinfo[id];
	tinfo->stat[status]++;
	tinfo->curr = status;

	if (status < THREAD_STATUS__KIND_SPIN)
	{
		thread_info_show_table();
	}
}

void thread_info_set_type(const uint32_t id, THREAD_TYPE type)
{
	thread_info *tinfo = &g_tinfo[id];
	tinfo->type = type;
}

