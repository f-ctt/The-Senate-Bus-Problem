# The Senate Bus Problem

Inspired by: [The Little Book of Semaphores](http://greenteapress.com/semaphores/LittleBookOfSemaphores.pdf)

This rep is for learning purposes - contains bugs, uncommented code, etc.

Output example:
```
BUS		    : start
BUS		    : arrival
BUS		    : depart
RID: 1		: start
RID: 1		: enter: 1
BUS		    : end
BUS		    : arrival
RID: 1		: boarding: 1
BUS		    : depart
RID: 2		: start
RID: 2		: enter: 1
BUS		    : end
RID: 1		: finish
BUS		    : arrival
RID: 2		: boarding: 1
BUS		    : depart
BUS		    : end
BUS		    : arrival
RID: 2		: finish
BUS		    : depart
RID: 3		: start
RID: 3		: enter: 1
BUS		    : end
BUS		    : arrival
RID: 3		: boarding: 1
BUS		    : depart
RID: 4		: start
RID: 4		: enter: 1
BUS		    : end
RID: 3		: finish
BUS		    : arrival
RID: 4		: boarding: 1
BUS		    : depart
RID: 5		: start
RID: 5		: enter: 1
RID: 6		: start
RID: 6		: enter: 2
RID: 7		: start
RID: 7		: enter: 3
BUS		    : end
BUS		    : arrival
RID: 4		: finish
RID: 8		: start
RID: 5		: boarding: 1
RID: 6		: boarding: 2
RID: 7		: boarding: 3
BUS		    : depart
RID: 8		: enter: 1
BUS		    : end
RID: 5		: finish
BUS		    : arrival
RID: 6		: finish
RID: 7		: finish
RID: 8		: boarding: 1
BUS		    : depart
RID: 9		: start
RID: 9		: enter: 1
BUS		    : end
RID: 8		: finish
BUS		    : arrival
RID: 9		: boarding: 1
BUS		    : depart
RID: 10		: start
RID: 10		: enter: 1
BUS		    : end
RID: 9		: finish
BUS		    : arrival
RID: 10		: boarding: 1
BUS		    : depart
RID: 10		: finish
BUS		    : end
BUS		    : finish
```
