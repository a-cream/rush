#include "../../include/interpreter/lexer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int tokens_capacity = 1;
static int tokens_size = 0;
static Token *tokens = NULL;
static unsigned int pos = 0;

char *strdup(const char *str) {
    size_t len = strlen(str) + 1;
    char *copy = malloc(len);
    if (copy == NULL) {
        return NULL;
    }
    memcpy(copy, str, len);
    return copy;
}

Token create_token(const TokenType type, const char *value) {
    Token token;
    token.type = type;
    token.value = strdup(value);
    if (token.value == NULL) {
        fprintf(stderr, "Memory allocation failed for token value\n");
        exit(EXIT_FAILURE);
    }
    return token;
}

void push(const Token value) {
    if (tokens_size == tokens_capacity) {
        int new_capacity = tokens_capacity * 2;
        Token *new_tokens =
            (Token *)realloc(tokens, new_capacity * sizeof(Token));
        if (new_tokens == NULL) {
            fprintf(
                stderr, "Memory allocation failed during token array resize\n"
            );
            exit(EXIT_FAILURE);
        }
        tokens = new_tokens;
        tokens_capacity = new_capacity;
    }

    tokens[tokens_size++] = value;
}

void cleanup(void) {
    if (tokens != NULL) {
        for (int i = 0; i < tokens_size; i++) {
            free((void *)tokens[i].value);
        }
        free(tokens);

        tokens = NULL;
        tokens_capacity = 1;
        tokens_size = 0;
    }
    pos = 0;
}

void advance(Lexer *self) {
    pos++;
    if (pos < strlen(self->src)) {
        self->current_char = self->src[pos];
    } else {
        self->current_char = '\0';
    }
}

Token *Lexer_lex(Lexer *self) {
    cleanup();
    self->current_char = self->src[pos];

    tokens = (Token *)malloc(tokens_capacity * sizeof(Token));
    if (tokens == NULL) {
        fprintf(stderr, "Memory allocation failed for tokens array\n");
        exit(EXIT_FAILURE);
    }

    char arg[256];

    while (self->current_char != '\0') {
        if (self->current_char == ' ') {
            advance(self);
            continue;
        }

        int arg_pos = 0;
        while (self->current_char != ' ' && self->current_char != '\0') {
            if (arg_pos >= 255) {
                fprintf(stderr, "Error: Word too long (max 255 characters)\n");
                exit(EXIT_FAILURE);
            }
            arg[arg_pos++] = self->current_char;
            advance(self);
        }

        if (arg_pos > 0) {
            arg[arg_pos] = '\0';
            Token token = create_token(ARG, arg);
            push(token);
        }
    }

    return tokens;
}

void Lexer_init(Lexer *self, const char *src) {
    self->src = src;
    self->current_char = self->src[pos];
}
