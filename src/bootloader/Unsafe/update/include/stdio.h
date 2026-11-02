#pragma once
typedef struct leanboot_unused_file FILE;
extern FILE *stderr;
int printf(const char *, ...);
int fprintf(FILE *, const char *, ...);
