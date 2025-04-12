#ifndef SHELL_H
#define SHELL_H

/* For now the struct implementation is a bit overkill
* In future implementations it will be usefull
*/
typedef struct {
    char *prompt;
} Shell;

void Shell_init(Shell *self, char *prompt);
char *Shell_interactive(Shell *self);

void shell_run(void);

#endif // !SHELL_H
