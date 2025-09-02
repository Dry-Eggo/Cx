
SRCDIR = src
BINDIR = bin

SRCS = ${wildcard ${SRCDIR}/*.c}
OBJ  = ${patsubst ${SRCDIR}/%.c, ${BINDIR}/%.o, ${SRCS}}

compiler = cxc

all: ${compiler}

${compiler}: ${OBJ}
	${CC} ${OBJ} -o $@

${BINDIR}/%.o: ${SRCDIR}/%.c
	${CC} -c $^ -o $@
