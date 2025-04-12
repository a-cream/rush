#include <stdio.h>
#include <stdlib.h>
#include "../include/shell.h"

void Shell_init(Shell *self, char *prompt) {
    self->prompt = prompt;
}

char *Shell_interactive(Shell *self) {
    int length = 256;
    char *str = (char *)malloc(length * sizeof(char));
    if (str == NULL) {
        perror("Memory allocation failed");
        exit(EXIT_FAILURE);
    }

    printf("%s ", self->prompt);
    if (fgets(str, length, stdin) == NULL) {
        free(str);
        return NULL;
    }
    return str;
}

void shell_run(void) {
    Shell shell;
    Shell_init(&shell, " >");

    for (;;) {
        char *input = Shell_interactive(&shell);
        if (input == NULL) {
            break;
        }

        printf("%s", input);
        free(input);
    }
}
