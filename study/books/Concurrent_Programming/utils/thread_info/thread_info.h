#ifndef __THREAD_INFO_H__
#define __THREAD_INFO_H__

#include <stdint.h>

typedef enum {
	THREAD_STATUS_NONE,
	THREAD_STATUS_START,
	THREAD_STATUS_WAIT,
	THREAD_STATUS_SPIN_ACQUIRE,
	THREAD_STATUS_READ_ACQUIRE,
	THREAD_STATUS_WRITE_ACQUIRE,
	THREAD_STATUS_WRITE_SPIN_ACQUIRE,
	THREAD_STATUS_SPIN_RELEASE,
	THREAD_STATUS_READ_RELEASE,
	THREAD_STATUS_WRITE_RELEASE,
	THREAD_STATUS_WRITE_SPIN_RELEASE,
	THREAD_STATUS_END,
	THREAD_STATUS_SPIN,
	THREAD_STATUS__KIND_SPIN = THREAD_STATUS_SPIN,
	THREAD_STATUS_READ_SPIN,
	THREAD_STATUS_WRITE_SPIN,
	THREAD_STATUS_MAX
} THREAD_STATUS;

typedef enum {
	THREAD_TYPE_NONE,
	THREAD_TYPE_WRITER,
	THREAD_TYPE_READER,
	THREAD_TYPE_BOTH,
	THREAD_TYPE_MAX
} THREAD_TYPE;

const char *thread_info_get_type_str(uint32_t id);
const char *thread_info_get_type_color(uint32_t id);
const char *thread_info_get_status_str(uint32_t id);
const char *thread_info_get_status_color(uint32_t id);
void thread_info_show_table(void);
void thread_info_set_status(const uint32_t id, THREAD_STATUS status);
void thread_info_set_type(const uint32_t id, THREAD_TYPE type);
void thread_info_init(uint32_t cnt);
void thread_info_deinit(void);

#endif
