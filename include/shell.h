#ifndef SHELL_H
#define SHELL_H

typedef struct {
    char *prompt;
} Shell;

void Shell_init(Shell *self, char *prompt);
char *Shell_interactive(Shell *self);

void shell_run();

#endif
