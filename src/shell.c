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
    fgets(str, length, stdin);
    return str;
}

void shell_run() {
    Shell shell;
    Shell_init(&shell, " >");

    while (1) {
        char *input = Shell_interactive(&shell);
        printf("%s", input);
        free(input);
    }
}
