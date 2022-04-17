CFLAGS=-std=c11 -g -fno-common -Wall -Wno-switch

SRCS=$(wildcard *.c)
OBJS=$(SRCS:.c=.o)


# Stage 1

titan: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

$(OBJS): titan.h

clean:
	rm -rf titan tmp*
	find * -type f '(' -name '*~' -o -name '*.o' ')' -exec rm {} ';'

